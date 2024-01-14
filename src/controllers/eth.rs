#![allow(clippy::unused_async)]
use axum::extract::RawQuery;
use loco_rs::prelude::*;
use reqwest;

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub chain_name: String,
    pub prefix: String,
    pub chain_symbol: String,
    pub api_url: String,
}

pub fn get_api_key(config: ScannerConfig) -> Result<String> {
    Ok(
        std::env::var(format!("{}_API_KEY", config.chain_symbol.to_uppercase())).map_err(|e| {
            println!("{} api key not found", config.chain_name);
            e
        })?,
    )
}

pub async fn forward_post(
    config: ScannerConfig,
    RawQuery(query): RawQuery,
    req_body: String,
) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}?{}&apikey={}",
        config.api_url,
        query.unwrap_or_default(),
        get_api_key(config.clone())?
    );
    let res = client
        .post(&url)
        .body(req_body)
        .send()
        .await
        .expect("error forwarding request to scanner")
        .text()
        .await;
    Ok(res.expect("error forwarding request to scanner"))
}

pub async fn forward_get(config: ScannerConfig, RawQuery(query): RawQuery) -> Result<String> {
    // forward request to etherscan
    let client = reqwest::Client::new();
    let url = format!(
        "{}?{}&apiKey={}",
        config.api_url,
        query.unwrap_or_default(),
        get_api_key(config.clone())?
    );
    let res = client
        .get(&url)
        .send()
        .await
        .expect("error forwarding request to scanner")
        .text()
        .await;
    Ok(res.expect("error forwarding request to scanner"))
}

pub fn scanner_routes(config: ScannerConfig) -> Routes {
    let clone = config.clone();
    Routes::new()
        .prefix(&format!("{}", config.prefix.to_lowercase()))
        .add(
            "/",
            get(move |query: RawQuery| forward_get(config.clone(), query)),
        )
        .add(
            "/",
            post(move |query: RawQuery, body: String| forward_post(clone.clone(), query, body)),
        )
}
