use std::time::Duration;

use reqwest::{Client, ClientBuilder, Error};
use serde_json::Value;

use super::Proxy;

pub async fn post_request(url: String, json: Value, proxy: &mut Proxy) -> Result<Value, Error> {
    Ok(build_client(proxy)?.post(url).json(&json).send().await?.json::<Value>().await?)
}

pub async fn get_request(url: String, proxy: &mut Proxy) -> Result<Value, Error> {
        Ok(build_client(proxy)?.get(url).send().await?.json::<Value>().await?)
}

pub fn build_client(proxy: &mut Proxy) -> Result<Client, Error> {
    ClientBuilder::default()
    .timeout(Duration::from_millis(1750))
    .user_agent("Mozilla/5.0")
    .proxy(
        reqwest::Proxy::all(format!("http://{}", proxy.combo))
            .unwrap()
    )
    .build()
}