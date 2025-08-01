// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: Vec<MetricResponse>,
    pub http_status_code: i32,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricResponse {
    pub metric_type: String,
    pub periods: Vec<Period>,
    pub host_id: String,
    pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub data: PeriodData,
    pub metric_time: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodData {
    pub wan: WanData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WanData {
    pub avg_latency: i32,
    #[serde(rename = "download_kbps")]
    pub download_kbps: i32,
    pub downtime: i32,
    pub isp_asn: String,
    pub isp_name: String,
    pub max_latency: i32,
    pub packet_loss: i32,
    #[serde(rename = "upload_kbps")]
    pub upload_kbps: i32,
    pub uptime: i32,
}