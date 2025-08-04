// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub id: Option<String>,
    pub fingerprint: Option<String>,
    pub updated_at: Option<i64>,
    pub hubs: Option<Vec<Hub>>,
    pub spokes: Option<Vec<Spoke>>,
    pub last_generated_at: Option<i64>,
    pub generate_status: Option<GenerateStatus>,
    pub errors: Option<Vec<serde_json::Value>>,
    pub warnings: Option<Vec<serde_json::Value>>,
    pub http_status_code: Option<i32>,
    pub trace_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hub {
    pub id: Option<String>,
    pub host_id: String,
    pub site_id: String,
    pub name: String,
    pub primary_wan_status: WanStatus,
    pub secondary_wan_status: WanStatus,
    pub errors: Vec<serde_json::Value>,
    pub warnings: Vec<serde_json::Value>,
    pub number_of_tunnels_used_by_other_features: i32,
    pub networks: Vec<Network>,
    pub routes: Vec<Route>,
    pub apply_status: ApplyStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spoke {
    pub id: Option<String>,
    pub host_id: String,
    pub site_id: String,
    pub name: String,
    pub primary_wan_status: WanStatus,
    pub secondary_wan_status: WanStatus,
    pub errors: Vec<serde_json::Value>,
    pub warnings: Vec<serde_json::Value>,
    pub number_of_tunnels_used_by_other_features: i32,
    pub networks: Vec<Network>,
    pub routes: Vec<Route>,
    pub connections: Vec<Connection>,
    pub apply_status: ApplyStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WanStatus {
    pub ip: String,
    pub latency: Option<f64>,
    pub internet_issues: Vec<serde_json::Value>,
    pub wan_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub network_id: String,
    pub name: String,
    pub errors: Vec<serde_json::Value>,
    pub warnings: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub route_value: String,
    pub errors: Vec<serde_json::Value>,
    pub warnings: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub hub_id: String,
    pub tunnels: Vec<Tunnel>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tunnel {
    pub spoke_wan_id: String,
    pub hub_wan_id: String,
    pub status: TunnelStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApplyStatus {
    Ok,
    Creating,
    Updating,
    Removing,
    CreateFailed,
    UpdateFailed,
    RemoveFailed,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerateStatus {
    Ok,
    Generating,
    GenerateFailed,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TunnelStatus {
    Connected,
    Disconnected,
    Pending,
    #[serde(other)]
    Unknown,
}
