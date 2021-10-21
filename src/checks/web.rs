use std::time::Duration;

use reqwest::{Client, ClientBuilder, Error};
use serde_json::Value;

use super::Proxy;

pub async fn post_request(url: String, json: Value, proxy: &mut Proxy) -> Result<Value, Error> {
    Ok(build_client(proxy)?
        .post(url)
        .json(&json)
        .send()
        .await?
        .json::<Value>()
        .await?)
}

pub async fn get_request(url: String, proxy: &mut Proxy) -> Result<Value, Error> {
    Ok(build_client(proxy)?
        .get(url)
        .send()
        .await?
        .json::<Value>()
        .await?)
}

pub fn build_client(proxy: &mut Proxy) -> Result<Client, Error> {
    let proxy_clone = proxy.clone();
    if let Some(username) = proxy_clone.combo.split(':').nth(2) {
        if let Some(password) = proxy_clone.combo.split(':').nth(3) {
            let proxy_string = format!(
                "http://{}:{}",
                proxy_clone.combo.split(':').next().unwrap(),
                proxy_clone.combo.split(':').nth(1).unwrap()
            );

            ClientBuilder::default()
                .timeout(Duration::from_millis(1750))
                .user_agent("Mozilla/5.0")
                .proxy(
                    reqwest::Proxy::all(proxy_string)
                        .unwrap()
                        .basic_auth(username, password),
                )
                .build()
        } else {
            Ok(Client::default())
        }
    } else {
        ClientBuilder::default()
            .timeout(Duration::from_millis(1750))
            .user_agent("Mozilla/5.0")
            .proxy(reqwest::Proxy::all(proxy_clone.combo).unwrap())
            .build()
    }
}
