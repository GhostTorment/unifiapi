use dotenvy::dotenv;
use std::env;

#[tokio::test]
async fn test_list_sites() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = sitemanager::endpoints::list_sites::list_sites(&api_key).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_sd_wan_configs() {
    dotenv().ok();
    let api_key = env::var("SITEMANAGER_API_KEY").expect("SITEMANAGER_API_KEY must be set");
    let result = sitemanager::endpoints::list_sd_wan_configs::list_sd_wan_configs(&api_key).await;
    assert!(result.is_ok());
}
