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
pub struct Sites {
    pub sites: Vec<Site>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub begin_timestamp: Option<DateTime<Utc>>,
    pub end_timestamp: Option<DateTime<Utc>>,
    pub host_id: String,
    pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: MetricsData,
    pub http_status_code: u16,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsData {
    pub metrics: Vec<Metric>,
    pub message: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub metric_type: String,
    pub periods: Vec<Period>,
    pub host_id: String,
    pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub data: PeriodData,
    pub metric_time: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodData {
    pub wan: WanMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WanMetrics {
    pub avg_latency: u32,
    #[serde(rename = "download_kbps")]
    pub download_kbps: u32,
    pub downtime: u32,
    pub isp_asn: String,
    pub isp_name: String,
    pub max_latency: u32,
    pub packet_loss: u32,
    #[serde(rename = "upload_kbps")]
    pub upload_kbps: u32,
    pub uptime: u32,
}
