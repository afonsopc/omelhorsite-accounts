use crate::config::CONFIG;
use crate::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct CountryResponse {
    country: String,
}

pub const UNKNOWN_COUNTRY: &str = "unknown";

pub async fn get_country_from_ip(ip: &str) -> String {
    let url = f!("{}/{}", CONFIG.ip_to_country_url, ip);
    let response = match reqwest::get(&url).await {
        Ok(response) => response,
        Err(_) => {
            log::error!("IP-TO-COUNTRY service unreachable.");
            return UNKNOWN_COUNTRY.to_string();
        }
    };

    match response
        .json::<CountryResponse>()
        .await
        .map(|response| response.country.to_lowercase())
    {
        Ok(country) => {
            if country == "none" {
                return UNKNOWN_COUNTRY.to_string();
            }
            country
        }
        Err(_) => UNKNOWN_COUNTRY.to_string(),
    }
}
