use anyhow::{Result, bail};
use std::path::Path;

pub fn validate(root: &Path) -> Result<()> {
  let (valid, errors, _) = arcantry_core::catalog::validate(root);
  if !valid {
    bail!(
      "Catalog and skill packages are invalid:\n- {}",
      errors.join("\n- ")
    );
  }
  println!("Catalog and skill packages are valid.");
  Ok(())
}
