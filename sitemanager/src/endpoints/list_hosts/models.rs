// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: Vec<Device>,
    pub http_status_code: i32,
    pub trace_id: String,
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: String,
    pub hardware_id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub ip_address: String,
    pub owner: bool,
    pub is_blocked: bool,
    pub registration_time: String,
    pub last_connection_state_change: DateTime<Utc>,
    pub latest_backup_time: String,
    pub user_data: UserData,
    pub reported_state: Option<serde_json::Value>,
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
    pub sys_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleAttributes {
    pub applications: HashMap<String, ApplicationAccess>,
    pub candidate_roles: Vec<String>,
    pub connected_state: String,
    pub connected_state_last_changed: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationAccess {
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