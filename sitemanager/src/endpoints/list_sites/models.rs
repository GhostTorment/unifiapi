use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: Vec<SiteData>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteData {
    pub site_id: String,
    pub host_id: String,
    pub meta: Meta,
    pub statistics: Statistics,
    pub permission: String,
    pub is_owner: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub desc: String,
    pub gateway_mac: String,
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub counts: Counts,
    pub gateway: Gateway,
    pub internet_issues: Vec<serde_json::Value>,
    pub isp_info: IspInfo,
    pub percentages: Percentages,
    pub wan_magic: WanMagic,
    pub wans: HashMap<String, WanInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Counts {
    pub critical_notification: u32,
    pub gateway_device: u32,
    pub guest_client: u32,
    pub lan_configuration: u32,
    pub offline_device: u32,
    pub offline_gateway_device: u32,
    pub offline_wifi_device: u32,
    pub offline_wired_device: u32,
    pub pending_update_device: u32,
    pub total_device: u32,
    pub wan_configuration: u32,
    pub wifi_client: u32,
    pub wifi_configuration: u32,
    pub wifi_device: u32,
    pub wired_client: u32,
    pub wired_device: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
    pub hardware_id: String,
    pub inspection_state: String,
    pub ips_mode: String,
    pub ips_signature: IpsSignature,
    pub shortname: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpsSignature {
    pub rules_count: u32,
    #[serde(rename = "type")]
    pub signature_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IspInfo {
    pub name: String,
    pub organization: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Percentages {
    pub tx_retry: f64,
    pub wan_uptime: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WanMagic {
    pub available: bool,
    pub enabled: bool,
    pub subscribed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WanInfo {
    #[serde(default)]
    pub external_ip: Option<String>,
    #[serde(default)]
    pub isp_info: Option<IspInfo>,
    pub wan_issues: Vec<WanIssue>,
    pub wan_uptime: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WanIssue {
    pub index: u32, // always present
    #[serde(default)]
    pub count: Option<u32>,
    #[serde(default)]
    pub wan_downtime: Option<bool>,
    #[serde(default)]
    pub packet_loss: Option<bool>,
    #[serde(default)]
    pub high_latency: Option<bool>,
    #[serde(default)]
    pub latency_avg_ms: Option<u32>,
    #[serde(default)]
    pub latency_max_ms: Option<u32>,
    #[serde(default)]
    pub failover_wan_active: Option<bool>,
}
