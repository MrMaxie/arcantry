use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

struct Archive {
  target: &'static str,
  name: &'static str,
}

const ARCHIVES: [Archive; 6] = [
  Archive {
    target: "x86_64-pc-windows-msvc",
    name: "arcantry-cli-x86_64-pc-windows-msvc.zip",
  },
  Archive {
    target: "aarch64-pc-windows-msvc",
    name: "arcantry-cli-aarch64-pc-windows-msvc.zip",
  },
  Archive {
    target: "x86_64-apple-darwin",
    name: "arcantry-cli-x86_64-apple-darwin.tar.xz",
  },
  Archive {
    target: "aarch64-apple-darwin",
    name: "arcantry-cli-aarch64-apple-darwin.tar.xz",
  },
  Archive {
    target: "x86_64-unknown-linux-musl",
    name: "arcantry-cli-x86_64-unknown-linux-musl.tar.xz",
  },
  Archive {
    target: "aarch64-unknown-linux-musl",
    name: "arcantry-cli-aarch64-unknown-linux-musl.tar.xz",
  },
];

const INSTALLERS: [(&str, &str); 2] = [
  ("arcantry-cli-installer.sh", "arcantry-installer.sh"),
  ("arcantry-cli-installer.ps1", "arcantry-installer.ps1"),
];

pub fn collect_release_artifacts(input: &Path, output: &Path) -> Result<()> {
  fs::create_dir_all(output)?;
  for archive in &ARCHIVES {
    let source = input
      .join(format!("native-{}", archive.target))
      .join("distrib");
    for name in [archive.name.to_owned(), format!("{}.sha256", archive.name)] {
      fs::copy(source.join(&name), output.join(&name)).with_context(|| {
        format!(
          "failed to collect {} for native target {}",
          name, archive.target
        )
      })?;
    }
  }
  println!("Collected native release archives in {}.", output.display());
  Ok(())
}

pub fn assemble_release(artifacts: &Path, installer_artifacts: &Path) -> Result<()> {
  if !artifacts.is_dir() {
    bail!(
      "cargo-dist artifact directory does not exist: {}",
      artifacts.display()
    );
  }
  let mut archive_digests = BTreeMap::new();
  for archive in &ARCHIVES {
    if !artifacts.join(archive.name).is_file() {
      bail!("required cargo-dist archive is missing: {}", archive.name);
    }
    let bytes = fs::read(artifacts.join(archive.name))?;
    let digest = sha256(&bytes);
    let cargo_dist_checksum =
      fs::read_to_string(artifacts.join(format!("{}.sha256", archive.name)))
        .with_context(|| format!("cargo-dist checksum is missing for {}", archive.name))?;
    if cargo_dist_checksum.split_whitespace().next() != Some(digest.as_str()) {
      bail!("cargo-dist checksum does not match {}", archive.name);
    }
    archive_digests.insert(archive.name.to_owned(), digest);
  }
  for (source, destination) in INSTALLERS {
    let content = fs::read_to_string(installer_artifacts.join(source))
      .with_context(|| format!("failed to read cargo-dist installer {source}"))?;
    let content = if source.ends_with(".sh") {
      patch_shell_installer(content, &archive_digests)?
    } else {
      patch_powershell_installer(content, &archive_digests)?
    };
    fs::write(artifacts.join(destination), content)
      .with_context(|| format!("failed to stage {destination} from cargo-dist output"))?;
  }

  let mut public_files = ARCHIVES
    .iter()
    .map(|archive| archive.name.to_owned())
    .collect::<Vec<_>>();
  public_files.extend(INSTALLERS.map(|(_, destination)| destination.to_owned()));
  public_files.sort();
  let checksums = public_files
    .iter()
    .map(|name| {
      let bytes = fs::read(artifacts.join(name))?;
      Ok(format!("{}  {name}", sha256(&bytes)))
    })
    .collect::<Result<Vec<_>>>()?
    .join("\n");
  fs::write(artifacts.join("SHA256SUMS"), format!("{checksums}\n"))?;

  println!(
    "Assembled six native archives, two installers, and SHA256SUMS in {}.",
    artifacts.display()
  );
  Ok(())
}

fn sha256(bytes: &[u8]) -> String {
  Sha256::digest(bytes)
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect()
}

fn patch_shell_installer(
  mut content: String,
  digests: &BTreeMap<String, String>,
) -> Result<String> {
  for (archive, digest) in digests {
    let marker = format!("        \"{archive}\")");
    let section = content
      .find(&marker)
      .with_context(|| format!("shell installer does not declare {archive}"))?;
    let zip = content[section..]
      .find("            _zip_ext=")
      .map(|offset| section + offset)
      .with_context(|| format!("shell installer has no archive metadata for {archive}"))?;
    let line_end = content[zip..]
      .find('\n')
      .map(|offset| zip + offset + 1)
      .context("shell installer archive metadata is truncated")?;
    content.insert_str(
      line_end,
      &format!(
        "            _checksum_style=\"sha256\"\n            _checksum_value=\"{digest}\"\n"
      ),
    );
  }
  let optional_verification = r#"        sha256)
            if ! check_cmd sha256sum; then
                say "skipping sha256 checksum verification (it requires the 'sha256sum' command)"
                return 0
            fi
            _calculated_checksum="$(sha256sum -b "$_file" | awk '{printf $1}')"
            ;;"#;
  let required_verification = r#"        sha256)
            if check_cmd sha256sum; then
                _calculated_checksum="$(sha256sum -b "$_file" | awk '{printf $1}')"
            elif check_cmd shasum; then
                _calculated_checksum="$(shasum -a 256 "$_file" | awk '{printf $1}')"
            elif check_cmd openssl; then
                _calculated_checksum="$(openssl dgst -sha256 "$_file" | awk '{printf $NF}')"
            else
                err "sha256 checksum verification requires sha256sum, shasum, or openssl"
            fi
            ;;"#;
  if !content.contains(optional_verification) {
    bail!("cargo-dist shell checksum implementation changed unexpectedly");
  }
  Ok(content.replacen(optional_verification, required_verification, 1))
}

fn patch_powershell_installer(
  mut content: String,
  digests: &BTreeMap<String, String>,
) -> Result<String> {
  for (archive, digest) in digests {
    let marker = format!("      \"artifact_name\" = \"{archive}\"\n");
    if !content.contains(&marker) {
      continue;
    }
    content = content.replace(
      &marker,
      &format!("{marker}      \"checksum\" = \"{digest}\"\n"),
    );
  }
  for archive in digests.keys().filter(|name| name.ends_with(".zip")) {
    if !content.contains(&format!("\"artifact_name\" = \"{archive}\"")) {
      bail!("PowerShell installer does not declare Windows archive {archive}");
    }
  }
  let download = "  Invoke-DownloadFile -client $wc -url $url -path $dir_path\n";
  let verification = r#"  Invoke-DownloadFile -client $wc -url $url -path $dir_path
  $expected_checksum = $info["checksum"]
  $actual_checksum = (Get-FileHash -Algorithm SHA256 -Path $dir_path).Hash.ToLowerInvariant()
  if ($actual_checksum -ne $expected_checksum) {
    throw "checksum mismatch`n            want: $expected_checksum`n            got:  $actual_checksum"
  }
"#;
  if !content.contains(download) {
    bail!("cargo-dist PowerShell download implementation changed unexpectedly");
  }
  Ok(content.replacen(download, verification, 1))
}
