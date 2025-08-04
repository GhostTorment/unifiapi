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
    pub data: Vec<SdWanConfigDetail>,
    pub http_status_code: i32,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdWanConfigDetail {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub config_type: String, // Expected: "sdwan-hbsp"
    pub variant: Option<String>, // Expected: distributed, failover, single
    pub settings: Option<Settings>,
    pub hubs: Option<Vec<Hub>>,
    pub spokes: Option<Vec<Spoke>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub hubs_interconnect: Option<bool>,
    pub spoke_to_hub_tunnels_mode: String, // maxResiliency, redundant, scalable
    pub spokes_auto_scale_and_nat_enabled: bool,
    pub spokes_auto_scale_and_nat_range: Option<String>,
    pub spokes_isolate: bool,
    pub spoke_standard_settings_enabled: bool,
    pub spoke_standard_settings_values: Option<SpokeStandardSettings>,
    pub spoke_to_hub_routing: Option<String>, // custom, geo
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpokeStandardSettings {
    pub primary_wan: Option<String>, // e.g., "WAN"
    pub wan_failover: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hub {
    pub id: String,
    pub host_id: String,
    pub site_id: String,
    pub network_ids: Vec<String>,
    pub routes: Vec<String>,
    pub primary_wan: String,
    pub wan_failover: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spoke {
    pub id: String,
    pub host_id: String,
    pub site_id: String,
    pub network_ids: Vec<String>,
    pub routes: Vec<String>,
    pub primary_wan: String,
    pub wan_failover: bool,
    pub hubs_priority: Option<Vec<String>>, // Only for distributed + custom
}
