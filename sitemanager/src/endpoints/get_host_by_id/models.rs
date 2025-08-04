// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: HostResponse,
    pub http_status_code: i32,
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostResponse {
    pub id: String,
    pub hardware_id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub ip_addr: Option<String>,
    pub owner: bool,
    pub is_blocked: bool,
    pub registration_time: DateTime<Utc>,
    pub last_connection_state_change: DateTime<Utc>,
    pub latest_backup_time: DateTime<Utc>,
    pub user_data: Option<UserData>,
    pub reported_state: Option<State>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub apps: Vec<String>,
    pub console_group_members: Vec<ConsoleGroupMember>,
    pub controllers: Vec<String>,
    pub email: String,
    pub features: Features,
    pub full_name: String,
    pub local_id: String,
    pub permissions: HashMap<String, Vec<String>>,
    pub role: String,
    pub role_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleGroupMember {
    pub mac: String,
    pub role: String,
    pub role_attributes: RoleAttributes,
    pub sys_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAttributes {
    pub applications: Applications,
    pub candidate_roles: Vec<String>,
    pub connected_state: String,
    pub connected_state_last_changed: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Applications {
    pub access: AppFeature,
    pub connect: AppFeature,
    pub innerspace: AppFeature,
    pub network: AppFeature,
    pub protect: AppFeature,
    pub talk: AppFeature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppFeature {
    pub owned: bool,
    pub required: bool,
    pub supported: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    pub device_groups: bool,
    pub floorplan: Floorplan,
    pub manage_applications: bool,
    pub notifications: bool,
    pub pion: Option<bool>,
    pub webrtc: WebRtc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Floorplan {
    pub can_edit: bool,
    pub can_view: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebRtc {
    pub ice_restart: bool,
    pub media_streams: bool,
    pub two_way_audio: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub controller_uuid: Option<String>,
    pub device_id: Option<String>,
    #[serde(rename = "firmware_version")]
    pub firmware_version: Option<String>,
    #[serde(rename = "hardware_id")]
    pub hardware_id: Option<String>,
    #[serde(rename = "host_type")]
    pub host_type: Option<u32>,
    pub hostname: Option<String>,
    #[serde(rename = "inform_port")]
    pub inform_port: Option<u16>,
    pub ip_addrs: Option<Vec<String>>,
    #[serde(rename = "mgmt_port")]
    pub mgmt_port: Option<u16>,
    pub name: Option<String>,
    #[serde(rename = "override_infom_host")]
    pub override_inform_host: Option<bool>,
    #[serde(rename = "release_channel")]
    pub release_channel: Option<String>,
    pub state: Option<String>,
    pub version: Option<String>,
}
