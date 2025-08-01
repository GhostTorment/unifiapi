// Copyright (c) 2025 Jak Bracegirdle
//
// This file is part of the unifiapi_sitemanager crate.
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>
// or the MIT license <http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use dotenvy::dotenv;
use std::env;
use unifiapi_sitemanager::endpoints::*;

#[tokio::test]
async fn test_list_sites() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = list_sites::list_sites(&api_key).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_hosts() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = list_hosts::list_hosts(&api_key).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_sd_wan_configs() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = list_sd_wan_configs::list_sd_wan_configs(&api_key).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_isp_metrics() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = get_isp_metrics::get_isp_metrics(&api_key, "5m", None, None, Option::from("24h")).await;
    println!("{:?}", result);
    assert!(result.is_ok());
}