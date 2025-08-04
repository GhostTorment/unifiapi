// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use crate::endpoints::list_devices::models::ApiResponse;
use reqwest;
use std::error::Error;

pub mod models;

pub async fn list_devices(
    api_key: &str,
    host_ids: Vec<&str>,
) -> Result<ApiResponse, Box<dyn Error>> {
    let q_host_ids = format!("[{}]", host_ids.join(","));

    let api_url = format!("https://api.ui.com/v1/devices?hostIds={q_host_ids}&pageSize=1000");

    println!("{}", api_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&api_url)
        .header("X-API-KEY", api_key)
        .header("Accept", "application/json")
        .send()
        .await?;

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
