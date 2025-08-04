// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use crate::endpoints::get_sd_wan_config_status::models::ApiResponse;
use reqwest;

pub mod models;

pub async fn get_sd_wan_config_status(
    api_key: &str,
    sdwan_id: &str,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let api_url = format!("https://api.ui.com/ea/sd-wan-configs?id={sdwan_id}");

    let client = reqwest::Client::new();

    let response = client
        .get(api_url)
        .header("X-API-KEY", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let json: ApiResponse = response.json().await?;
            Ok(json)
        }
        _ => {
            let status_code = response.status();
            let err_text = response.text().await?;
            Err(format!("Request failed: {} - {}", status_code, err_text).into())
        }
    }
}
