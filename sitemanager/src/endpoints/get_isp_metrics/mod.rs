use std::error::Error;
use chrono::{DateTime, Utc};
use crate::endpoints::get_isp_metrics::models::{ApiResponse};
use reqwest;

pub mod models;

pub async fn get_isp_metrics(
    api_key: &str,
    interval: &str,
    begin_timestamp: Option<DateTime<Utc>>,
    end_timestamp: Option<DateTime<Utc>>,
    duration: Option<&str>,
) -> Result<ApiResponse, Box<dyn Error>> {
    if interval == "5m" || interval == "1h" {
        let mut api_url = format!("https://api.ui.com/ea/isp-metrics/{interval}");

        // Validate parameter combination
        match (begin_timestamp, end_timestamp, duration) {
            (Some(begin), Some(end), None) => {
                api_url.push_str(&format!(
                    "?startTimestamp={}&endTimestamp={}",
                    begin.to_rfc3339(),
                    end.to_rfc3339()
                ));
            }
            (None, None, Some(dur)) => {
                if (interval == "5m" && dur == "24h") || (interval == "1h" && (dur == "7d" || dur == "30d")) {
                    api_url.push_str(&format!("?duration={}", dur));
                } else {
                    return Err("Invalid duration specified for query, Supports 24h for 5-minute metrics, and 7d or 30d for 1-hour metrics.".into());
                }

            }
            _ => {
                return Err("Invalid combination of time parameters: provide either both begin & end timestamps or just duration".into());
            }
        }

        let client = reqwest::Client::new();
        let response = client
            .get(&api_url)
            .header("X-API-KEY", api_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let json: ApiResponse = response.json().await?;
                Ok(json)
            }
            _ => {
                let status_code = response.status();
                let err_text = response.text().await?;
                Err(format!("Request failed: {} - {}", status_code, err_text).into())
            }
        }
    } else {
        Err("Request failed: Interval must be one of 5m,1h".into())
    }
}