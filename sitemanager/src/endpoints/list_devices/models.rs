// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub code: Option<i32>,
    pub data: Vec<HostData>,
    pub http_status_code: i32,
    pub trace_id: String,
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostData {
    pub host_id: String,
    pub host_name: String,
    pub devices: Vec<Device>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    pub mac: String,
    pub name: String,
    pub model: String,
    pub shortname: String,
    pub ip: String,
    pub product_line: Option<String>,
    pub status: String,
    pub version: String,
    pub firmware_status: String,
    pub update_available: Option<String>,
    pub is_console: Option<bool>,
    pub is_managed: Option<bool>,
    pub startup_time: Option<DateTime<Utc>>,
    pub adoption_time: Option<DateTime<Utc>>,
    pub note: Option<String>,
    pub uidb: Option<serde_json::Value>, // Adjust to a custom struct if the contents are known
}
