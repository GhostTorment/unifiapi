use crate::endpoints::list_sd_wan_configs::models::ApiResponse;
use reqwest;

pub mod models;

pub async fn list_sd_wan_configs(api_key: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    static API_URL: &str = "https://api.ui.com/ea/sd-wan-configs";

    let client = reqwest::Client::new();

    let response = client.get(API_URL)
        .header("X-API-KEY", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let json: ApiResponse = response.json().await.unwrap();
            Ok(json)
        }
        _ => {
            let status_code = response.status();
            let err_text = response.text().await?;
            Err(format!("Request failed: {} - {}", status_code, err_text).into())
        }
    }
}