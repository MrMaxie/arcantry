use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

const INSPECTED_EXTENSIONS: [&str; 8] = ["css", "html", "js", "json", "md", "svg", "tosd", "xml"];
const STALE_PATTERNS: [&str; 7] = [
  "https://maxie.dev/arcantry/",
  "https://mrmaxie.github.io/arcantry/",
  "href=\"/arcantry/",
  "src=\"/arcantry/",
  "href='/arcantry/",
  "src='/arcantry/",
  "/_astro/",
];

pub fn verify(root: &Path) -> Result<()> {
  let output_root = root.join("apps/docs/dist");
  let failures = inspect(&output_root)?;
  if !failures.is_empty() {
    bail!("- {}", failures.join("\n- "));
  }
  println!("Documentation output uses arcantry.dev and content-hashed /static/ assets.");
  Ok(())
}

fn inspect(output_root: &Path) -> Result<Vec<String>> {
  let mut failures = Vec::new();
  for path in files(output_root)? {
    let extension = path.extension().and_then(|value| value.to_str());
    if !extension.is_some_and(|value| INSPECTED_EXTENSIONS.contains(&value)) {
      continue;
    }
    let content = fs::read_to_string(&path)
      .with_context(|| format!("could not read documentation output {}", path.display()))?;
    for pattern in STALE_PATTERNS {
      if content.contains(pattern) {
        let relative = path.strip_prefix(output_root).unwrap_or(&path);
        failures.push(format!("{} contains {pattern:?}", relative.display()));
      }
    }
  }

  let homepage = read(output_root.join("index.html"))?;
  if !homepage.contains("<link rel=\"canonical\" href=\"https://arcantry.dev/\"") {
    failures.push("index.html does not declare https://arcantry.dev/ as canonical".to_owned());
  }
  if !homepage.contains("<meta property=\"og:url\" content=\"https://arcantry.dev/\"") {
    failures
      .push("index.html does not declare https://arcantry.dev/ as its Open Graph URL".to_owned());
  }

  let sitemap_index = read(output_root.join("sitemap-index.xml"))?;
  if !sitemap_index.contains("<loc>https://arcantry.dev/sitemap-") {
    failures.push("sitemap-index.xml does not reference the arcantry.dev sitemap".to_owned());
  }

  let static_path = output_root.join("static");
  if !static_path.is_dir() || files(&static_path)?.is_empty() {
    failures.push("Astro did not emit generated assets under dist/static".to_owned());
  }
  if output_root.join("_astro").is_dir() {
    failures.push("Astro emitted the deprecated dist/_astro asset directory".to_owned());
  }
  Ok(failures)
}

fn read(path: PathBuf) -> Result<String> {
  fs::read_to_string(&path)
    .with_context(|| format!("could not read documentation output {}", path.display()))
}

fn files(directory: &Path) -> Result<Vec<PathBuf>> {
  if !directory.is_dir() {
    return Ok(Vec::new());
  }
  let mut found = Vec::new();
  for entry in fs::read_dir(directory).with_context(|| {
    format!(
      "could not inspect documentation output {}",
      directory.display()
    )
  })? {
    let path = entry?.path();
    if path.is_dir() {
      found.extend(files(&path)?);
    } else if path.is_file() {
      found.push(path);
    }
  }
  found.sort();
  Ok(found)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn valid_output(root: &Path) -> PathBuf {
    let output = root.join("apps/docs/dist");
    fs::create_dir_all(output.join("static")).unwrap();
    fs::write(
      output.join("index.html"),
      "<link rel=\"canonical\" href=\"https://arcantry.dev/\"><meta property=\"og:url\" content=\"https://arcantry.dev/\"><script src=\"/static/app.js\"></script>",
    )
    .unwrap();
    fs::write(
      output.join("sitemap-index.xml"),
      "<loc>https://arcantry.dev/sitemap-0.xml</loc>",
    )
    .unwrap();
    fs::write(output.join("static/app.js"), "console.log('ok');").unwrap();
    output
  }

  #[test]
  fn accepts_the_documentation_output_contract() {
    let root = tempfile::tempdir().unwrap();
    let output = valid_output(root.path());
    assert!(inspect(&output).unwrap().is_empty());
  }

  #[test]
  fn reports_stale_origins_and_asset_layouts() {
    let root = tempfile::tempdir().unwrap();
    let output = valid_output(root.path());
    fs::write(
      output.join("stale.html"),
      "<a href=\"https://maxie.dev/arcantry/\">old</a>",
    )
    .unwrap();
    fs::create_dir(output.join("_astro")).unwrap();

    let failures = inspect(&output).unwrap();
    assert!(
      failures
        .iter()
        .any(|failure| failure.contains("https://maxie.dev/arcantry/"))
    );
    assert!(
      failures
        .iter()
        .any(|failure| failure.contains("deprecated dist/_astro"))
    );
  }
}
