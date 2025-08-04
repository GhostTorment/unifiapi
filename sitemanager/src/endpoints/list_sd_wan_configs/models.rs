// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub data: Vec<ConfigSummary>,
    pub http_status_code: u16,
    pub trace_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSummary {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub config_type: String,
}
