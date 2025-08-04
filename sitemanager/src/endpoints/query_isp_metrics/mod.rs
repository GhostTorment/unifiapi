// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use crate::endpoints::query_isp_metrics::models::{ApiResponse, Sites};
use reqwest;
use std::error::Error;

pub mod models;

pub async fn query_isp_metrics(
    api_key: &str,
    interval: &str,
    sites: &Sites,
) -> Result<ApiResponse, Box<dyn Error>> {
    if interval == "5m" || interval == "1h" {
        let api_url = format!("https://api.ui.com/ea/isp-metrics/{interval}/query");

        let client = reqwest::Client::new();
        let response = client
            .post(&api_url)
            .header("X-API-KEY", api_key)
            .header("Accept", "application/json")
            .body(serde_json::to_string(sites).unwrap())
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
    } else {
        Err("Request failed: Interval must be one of 5m,1h".into())
    }
}
