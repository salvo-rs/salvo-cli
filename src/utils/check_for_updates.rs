use crate::utils::{print_util::orange, success, warning};
use rust_i18n::t;
use serde_json::Value;
use std::time::Duration;
use tokio::time::timeout;

use super::print_util::green;

async fn get_latest_version(crate_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let client = reqwest::Client::builder()
        .user_agent("salvo-cli update checker")
        .build()?;
    let resp = client.get(&url).send().await?;
    let body: String = resp.text().await?;
    let crate_response: Value = serde_json::from_str(&body)?;
    let latest_version = crate_response["crate"]["max_version"]
        .as_str()
        .ok_or("Failed to get max_version")?
        .to_string();
    Ok(latest_version)
}

pub async fn check_for_updates() {
    success(t!("checking_for_updates"));
    let result = timeout(Duration::from_secs(3), get_latest_version("salvo-cli")).await;

    match result {
        Ok(Ok(latest_version)) => {
            let current_version = env!("CARGO_PKG_VERSION");
            if latest_version != current_version {
                orange(t!("new_version_available", latest_version = latest_version));
                orange(t!(
                    "currently_using_version",
                    current_version = current_version
                ));
                orange(t!("consider_updating"));
            } else {
                green(t!("current_version_up_to_date"));
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
