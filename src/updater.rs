use std::env;
use std::time::Duration;

use rust_i18n::t;
use semver::Version;
use serde::Deserialize;
use serde_json::from_str;
use tokio::time::timeout;

use crate::printer::{green, orange, success, warning};

#[derive(Debug, Deserialize)]
struct CratesResponse {
    #[serde(rename = "crate")]
    package: CratesPackage,
}

#[derive(Debug, Deserialize)]
struct CratesPackage {
    max_version: String,
}

#[derive(Debug, PartialEq, Eq)]
enum UpdateStatus {
    UpToDate,
    UpdateAvailable(Version),
}

async fn get_latest_version(crate_name: &str) -> Result<Version, Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = reqwest::Client::builder()
        .user_agent("salvo-cli update checker")
        .build()?;

    let resp = client.get(&url).send().await?.error_for_status()?;
    let body = resp.text().await?;
    let crate_response: CratesResponse = from_str(&body)?;
    Ok(Version::parse(&crate_response.package.max_version)?)
}

fn should_skip_update_check() -> bool {
    parse_skip_update_env(env::var("SALVO_SKIP_UPDATE_CHECK").ok().as_deref())
}

fn parse_skip_update_env(value: Option<&str>) -> bool {
    matches!(value, Some(value) if !matches!(value.trim(), "" | "0" | "false" | "False" | "FALSE"))
}

fn resolve_update_status(
    current_version: &str,
    latest_version: Version,
) -> Result<UpdateStatus, semver::Error> {
    let current_version = Version::parse(current_version)?;
    if latest_version > current_version {
        Ok(UpdateStatus::UpdateAvailable(latest_version))
    } else {
        Ok(UpdateStatus::UpToDate)
    }
}

pub async fn check_for_updates() {
    if should_skip_update_check() {
        return;
    }
    success(t!("checking_for_updates"));
    let result = timeout(Duration::from_secs(3), get_latest_version("salvo-cli")).await;

    match result {
        Ok(Ok(latest_version)) => {
            let current_version = env!("CARGO_PKG_VERSION");
            match resolve_update_status(current_version, latest_version) {
                Ok(UpdateStatus::UpdateAvailable(latest_version)) => {
                    orange(t!("new_version_available", latest_version = latest_version));
                    orange(t!(
                        "currently_using_version",
                        current_version = current_version
                    ));
                    orange(t!("consider_updating"));
                }
                Ok(UpdateStatus::UpToDate) => {
                    green(t!("current_version_up_to_date"));
                }
                Err(e) => {
                    warning(format!("{},{}", t!("unable_to_verify_updates"), e));
                }
            }
        }
        Ok(Err(e)) => {
            warning(format!("{},{}", t!("unable_to_verify_updates"), e));
        }
        Err(_) => {
            warning(t!("update_verification_took_long"));
        }
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::{UpdateStatus, parse_skip_update_env, resolve_update_status};

    #[test]
    fn resolve_update_status_detects_newer_release() {
        let status = resolve_update_status("0.3.0", Version::parse("0.4.0").unwrap()).unwrap();
        assert_eq!(status, UpdateStatus::UpdateAvailable(Version::new(0, 4, 0)));
    }

    #[test]
    fn resolve_update_status_treats_equal_or_newer_local_builds_as_up_to_date() {
        let equal = resolve_update_status("0.3.0", Version::parse("0.3.0").unwrap()).unwrap();
        assert_eq!(equal, UpdateStatus::UpToDate);

        let newer_local =
            resolve_update_status("0.4.0-dev.1", Version::parse("0.3.0").unwrap()).unwrap();
        assert_eq!(newer_local, UpdateStatus::UpToDate);
    }

    #[test]
    fn resolve_update_status_rejects_invalid_current_versions() {
        assert!(resolve_update_status("invalid", Version::new(0, 3, 0)).is_err());
    }

    #[test]
    fn parse_skip_update_env_honors_truthy_and_falsy_values() {
        assert!(parse_skip_update_env(Some("1")));
        assert!(parse_skip_update_env(Some("true")));
        assert!(!parse_skip_update_env(None));
        assert!(!parse_skip_update_env(Some("0")));
        assert!(!parse_skip_update_env(Some("false")));
        assert!(!parse_skip_update_env(Some("   ")));
    }
}
