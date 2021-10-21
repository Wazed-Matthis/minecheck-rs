
use reqwest::Error;
use serde_json::json;

use crate::{checks::web, types::Account};

use super::Proxy;


pub async fn check(account: &mut Account, proxy: Proxy) -> Result<(), Error> {
    let mojang_url = "https://authserver.mojang.com/authenticate";
    let response = web::post_request(
        mojang_url.to_string(),
        json!({
            "agent": {
                "name": "Minecraft",
                "version": 1
            },
            "username": account.email,
            "password": account.password,
            "requestUser": "true"
        }),
        proxy,
    )
    .await?;

    account.banned = response["selectedProfile"].is_null();
    account.access_token = response["selectedProfile"]["access_token"].to_string();
    account.username = response["selectedProfile"]["name"].to_string();
    account.uuid = response["selectedProfile"]["id"].to_string();
    Ok(())
}
