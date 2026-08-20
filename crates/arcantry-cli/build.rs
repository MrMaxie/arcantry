use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize)]
struct AssetManifestEntry {
  path: String,
  sha256: String,
}

fn main() {
  let crate_root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
  let repository_root = crate_root.join("../..");
  let roots = [
    (
      repository_root.join("catalog.json"),
      PathBuf::from("catalog.json"),
    ),
    (repository_root.join("skills"), PathBuf::from("skills")),
    (repository_root.join("schemas"), PathBuf::from("schemas")),
    (
      repository_root.join("openspec/schemas/arcantry"),
      PathBuf::from("openspec/schemas/arcantry"),
    ),
  ];
  let manifest = collect_asset_manifest(roots);
  let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("asset-manifest.json");
  fs::write(
    output,
    format!(
      "{}\n",
      serde_json::to_string_pretty(&manifest).expect("serialize asset manifest")
    ),
  )
  .expect("write asset manifest");
}

fn collect_asset_manifest<const N: usize>(
  roots: [(PathBuf, PathBuf); N],
) -> Vec<AssetManifestEntry> {
  let mut entries = Vec::new();
  for (source, target) in roots {
    println!("cargo:rerun-if-changed={}", source.display());
    if source.is_file() {
      entries.push(AssetManifestEntry {
        path: target.to_string_lossy().replace('\\', "/"),
        sha256: digest(&source),
      });
      continue;
    }
    for entry in WalkDir::new(&source)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|entry| entry.file_type().is_file())
    {
      let relative = entry
        .path()
        .strip_prefix(&source)
        .expect("embedded asset root");
      entries.push(AssetManifestEntry {
        path: target.join(relative).to_string_lossy().replace('\\', "/"),
        sha256: digest(entry.path()),
      });
    }
  }
  entries.sort_by(|left, right| left.path.cmp(&right.path));
  entries
}

fn digest(path: &Path) -> String {
  let bytes = fs::read(path).expect("read embedded asset");
  Sha256::digest(bytes)
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect()
}
