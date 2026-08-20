use anyhow::{Context, Result, bail};
use directories::ProjectDirs;
use fs4::FileExt;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn openspec_asset_files() -> Result<Vec<(String, String)>> {
  OpenSpec::iter()
    .map(|name| {
      let asset =
        OpenSpec::get(&name).with_context(|| format!("Missing embedded OpenSpec asset: {name}"))?;
      let content = String::from_utf8(asset.data.into_owned())
        .with_context(|| format!("Embedded OpenSpec asset is not UTF-8: {name}"))?;
      Ok((format!("schemas/arcantry/{name}"), content))
    })
    .collect()
}

pub fn public_catalog() -> Result<arcantry_core::catalog::Catalog> {
  let catalog: arcantry_core::catalog::Catalog = serde_json::from_slice(CATALOG)?;
  if catalog.schema != "./schemas/catalog.schema.json" {
    bail!("catalog.json has an unsupported schema.");
  }
  Ok(catalog)
}

pub fn public_skill(
  name: &str,
) -> Result<(
  arcantry_core::catalog::CatalogEntry,
  arcantry_core::catalog::SkillMetadata,
)> {
  let entry = public_catalog()?
    .skills
    .into_iter()
    .find(|entry| entry.name == name)
    .with_context(|| format!("Skill is not present in catalog.json: {name}"))?;
  let path = format!("{name}/arcantry.json");
  let asset =
    Skills::get(&path).with_context(|| format!("Missing embedded skill asset: {path}"))?;
  let metadata: arcantry_core::catalog::SkillMetadata =
    serde_json::from_slice(asset.data.as_ref())?;
  if metadata.schema != "../../schemas/skill-metadata.schema.json" {
    bail!("skills/{name}/arcantry.json has an unsupported schema.");
  }
  Ok((entry, metadata))
}

#[derive(RustEmbed)]
#[folder = "../../skills"]
struct Skills;
#[derive(RustEmbed)]
#[folder = "../../schemas"]
struct Schemas;
#[derive(RustEmbed)]
#[folder = "../../openspec/schemas/arcantry"]
struct OpenSpec;

const CATALOG: &[u8] = include_bytes!("../../../catalog.json");
const MANIFEST: &str = include_str!(concat!(env!("OUT_DIR"), "/asset-manifest.json"));
const OWNERSHIP_FILE: &str = ".arcantry-owned.json";

#[derive(Debug, Deserialize)]
struct AssetManifestEntry {
  path: String,
  sha256: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OwnershipMarker {
  product: String,
  version: String,
  manifest_sha256: String,
}

pub fn materialize_catalog() -> Result<PathBuf> {
  let directories = ProjectDirs::from("dev", "MrMaxie", "Arcantry")
    .context("Could not resolve the Arcantry data directory.")?;
  let parent = directories.data_dir().join("catalog");
  materialize_in(&parent)
}

fn materialize_in(parent: &Path) -> Result<PathBuf> {
  let target = parent.join(arcantry_core::VERSION);
  fs::create_dir_all(parent)?;
  let lock_path = parent.join(format!("{}.lock", arcantry_core::VERSION));
  let lock = OpenOptions::new()
    .create(true)
    .truncate(false)
    .read(true)
    .write(true)
    .open(lock_path)?;
  FileExt::lock(&lock)?;
  let result = materialize_locked(parent, &target);
  let _ = FileExt::unlock(&lock);
  result
}

fn materialize_locked(parent: &Path, target: &Path) -> Result<PathBuf> {
  if target.exists() {
    if verify(target).is_ok() {
      return Ok(target.to_path_buf());
    }
    if !is_owned_materialization(target) {
      bail!(
        "Arcantry catalog cache contains unexpected unowned content: {}",
        target.display()
      );
    }
    fs::remove_dir_all(target)?;
  }
  let temporary = tempfile::Builder::new()
    .prefix(&format!(".{}-", arcantry_core::VERSION))
    .tempdir_in(parent)?;
  write_assets(temporary.path())?;
  fs::write(temporary.path().join("asset-manifest.json"), MANIFEST)?;
  fs::write(
    temporary.path().join(OWNERSHIP_FILE),
    format!("{}\n", serde_json::to_string_pretty(&ownership_marker())?),
  )?;
  verify(temporary.path())?;
  let persisted = temporary.keep();
  fs::rename(persisted, target)?;
  verify(target)?;
  Ok(target.to_path_buf())
}

fn write_assets(root: &Path) -> Result<()> {
  write(root, "catalog.json", CATALOG)?;
  for name in Skills::iter() {
    let asset =
      Skills::get(&name).with_context(|| format!("Missing embedded skill asset: {name}"))?;
    write(root, &format!("skills/{name}"), asset.data.as_ref())?;
  }
  for name in Schemas::iter() {
    let asset =
      Schemas::get(&name).with_context(|| format!("Missing embedded schema asset: {name}"))?;
    write(root, &format!("schemas/{name}"), asset.data.as_ref())?;
  }
  for name in OpenSpec::iter() {
    let asset =
      OpenSpec::get(&name).with_context(|| format!("Missing embedded OpenSpec asset: {name}"))?;
    write(
      root,
      &format!("openspec/schemas/arcantry/{name}"),
      asset.data.as_ref(),
    )?;
  }
  Ok(())
}

fn write(root: &Path, relative: &str, content: &[u8]) -> Result<()> {
  let path = root.join(relative);
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, content)?;
  Ok(())
}

fn verify(root: &Path) -> Result<()> {
  let entries: Vec<AssetManifestEntry> = serde_json::from_str(MANIFEST)?;
  let mut expected_files =
    std::collections::BTreeSet::from([OWNERSHIP_FILE.to_owned(), "asset-manifest.json".to_owned()]);
  for entry in entries {
    let actual = Sha256::digest(fs::read(root.join(&entry.path))?)
      .iter()
      .map(|byte| format!("{byte:02x}"))
      .collect::<String>();
    if actual != entry.sha256 {
      bail!("Embedded asset digest mismatch: {}", entry.path);
    }
    expected_files.insert(entry.path);
  }
  let actual: std::collections::BTreeSet<_> = WalkDir::new(root)
    .min_depth(1)
    .into_iter()
    .collect::<std::result::Result<Vec<_>, _>>()?
    .into_iter()
    .filter(|entry| entry.file_type().is_file())
    .map(|entry| {
      entry
        .path()
        .strip_prefix(root)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/")
    })
    .collect();
  if actual != expected_files {
    bail!("Embedded asset cache contains unexpected or missing files.");
  }
  Ok(())
}

fn ownership_marker() -> OwnershipMarker {
  OwnershipMarker {
    product: "arcantry".to_owned(),
    version: arcantry_core::VERSION.to_owned(),
    manifest_sha256: manifest_digest(),
  }
}

fn is_owned_materialization(target: &Path) -> bool {
  fs::read_to_string(target.join(OWNERSHIP_FILE))
    .ok()
    .and_then(|content| serde_json::from_str::<OwnershipMarker>(&content).ok())
    .is_some_and(|marker| {
      marker.product == "arcantry"
        && marker.version == arcantry_core::VERSION
        && marker.manifest_sha256 == manifest_digest()
    })
}

fn manifest_digest() -> String {
  Sha256::digest(MANIFEST.as_bytes())
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn materializes_reuses_and_repairs_owned_catalog() {
    let temporary = tempfile::tempdir().unwrap();
    let target = materialize_in(temporary.path()).unwrap();
    assert_eq!(target, materialize_in(temporary.path()).unwrap());
    fs::write(target.join("catalog.json"), "corrupt").unwrap();
    assert_eq!(target, materialize_in(temporary.path()).unwrap());
    verify(&target).unwrap();
  }

  #[test]
  fn refuses_unowned_content() {
    let temporary = tempfile::tempdir().unwrap();
    let target = temporary.path().join(arcantry_core::VERSION);
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("foreign.txt"), "keep").unwrap();
    assert!(
      materialize_in(temporary.path())
        .unwrap_err()
        .to_string()
        .contains("unowned content")
    );
    assert_eq!(
      fs::read_to_string(target.join("foreign.txt")).unwrap(),
      "keep"
    );
  }

  #[test]
  fn concurrent_materialization_uses_one_verified_target() {
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().to_path_buf();
    let handles: Vec<_> = (0..2)
      .map(|_| {
        let root = root.clone();
        std::thread::spawn(move || materialize_in(&root).unwrap())
      })
      .collect();
    let targets: Vec<_> = handles
      .into_iter()
      .map(|handle| handle.join().unwrap())
      .collect();
    assert_eq!(targets[0], targets[1]);
    verify(&targets[0]).unwrap();
  }

  #[test]
  fn ignores_an_unowned_interrupted_temporary_directory() {
    let temporary = tempfile::tempdir().unwrap();
    let interrupted = temporary.path().join(".1.0.0-interrupted");
    fs::create_dir_all(&interrupted).unwrap();
    fs::write(interrupted.join("foreign.txt"), "keep").unwrap();

    let target = materialize_in(temporary.path()).unwrap();

    verify(&target).unwrap();
    assert_eq!(
      fs::read_to_string(interrupted.join("foreign.txt")).unwrap(),
      "keep"
    );
  }
}
