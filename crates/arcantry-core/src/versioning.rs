//! Explicit project version conventions without a strategy framework.
use anyhow::{Result, bail};
use chrono::{Datelike, NaiveDate};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionStrategy {
  #[default]
  Semver,
  Integer,
  Calendar,
}

impl VersionStrategy {
  pub fn initial(self) -> &'static str {
    match self {
      Self::Semver => "0.0.0",
      Self::Integer => "0",
      Self::Calendar => "1970.01.01.1",
    }
  }

  /// A comparable key within one configured strategy, never a published identifier.
  pub fn key(self, value: &str) -> Result<Version> {
    match self {
      Self::Semver => {
        let parsed = Version::parse(value)?;
        if !parsed.pre.is_empty() || !parsed.build.is_empty() {
          bail!("Release version must be full stable SemVer: {value}.");
        }
        Ok(parsed)
      }
      Self::Integer => {
        let number: u64 = value.parse()?;
        if number.to_string() != value {
          bail!("Integer versions must be canonical non-negative numbers.");
        }
        Ok(Version::new(number, 0, 0))
      }
      Self::Calendar => {
        let (date, revision) = value
          .rsplit_once('.')
          .ok_or_else(|| anyhow::anyhow!("Calendar version must use YYYY.MM.DD.N."))?;
        let parsed = NaiveDate::parse_from_str(date, "%Y.%m.%d")?;
        let revision: u64 = revision.parse()?;
        if parsed.year() < 1970
          || parsed.year() > 9999
          || revision == 0
          || format!("{}.{revision}", parsed.format("%Y.%m.%d")) != value
        {
          bail!("Calendar version must use YYYY.MM.DD.N with a positive revision.");
        }
        Ok(Version::new(
          parsed.year() as u64 * 10000 + parsed.month() as u64 * 100 + parsed.day() as u64,
          revision,
          0,
        ))
      }
    }
  }

  pub fn next(self, current: &str, impact: &str, date: &str, changed: bool) -> Result<String> {
    let mut key = self.key(current)?;
    if !changed {
      return Ok(current.to_owned());
    }
    let increment = |n: u64| {
      n.checked_add(1)
        .ok_or_else(|| anyhow::anyhow!("Version counter overflow."))
    };
    match self {
      Self::Semver => {
        match impact {
          "major" => {
            key.major = increment(key.major)?;
            key.minor = 0;
            key.patch = 0;
          }
          "minor" => {
            key.minor = increment(key.minor)?;
            key.patch = 0;
          }
          "patch" => key.patch = increment(key.patch)?,
          _ => bail!("SemVer releases require a patch, minor or major impact."),
        }
        Ok(key.to_string())
      }
      Self::Integer => Ok(increment(key.major)?.to_string()),
      Self::Calendar => {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
        let prefix = date.format("%Y.%m.%d");
        let first = format!("{prefix}.1");
        let date_key = self.key(&first)?;
        if date_key.major < key.major {
          bail!("Release date precedes the current calendar version.");
        }
        Ok(format!(
          "{prefix}.{}",
          if date_key.major == key.major {
            increment(key.minor)?
          } else {
            1
          }
        ))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn advances_each_convention_and_rejects_calendar_rollback() {
    for (strategy, current, impact, date, expected) in [
      (
        VersionStrategy::Semver,
        "1.2.3",
        "minor",
        "2026-09-10",
        "1.3.0",
      ),
      (
        VersionStrategy::Integer,
        "9",
        "unspecified",
        "2026-09-10",
        "10",
      ),
      (
        VersionStrategy::Calendar,
        "2026.09.10.1",
        "unspecified",
        "2026-09-10",
        "2026.09.10.2",
      ),
      (
        VersionStrategy::Calendar,
        "2026.09.10.2",
        "unspecified",
        "2026-09-11",
        "2026.09.11.1",
      ),
    ] {
      assert_eq!(
        strategy.next(current, impact, date, true).unwrap(),
        expected
      );
    }
    assert!(
      VersionStrategy::Calendar
        .next("2026.09.10.1", "patch", "2026-09-09", true)
        .is_err()
    );
    assert!(VersionStrategy::Integer.key("01").is_err());
    assert!(VersionStrategy::Calendar.key("2026.02.30.1").is_err());
  }
}
