use crate::types::{Account, AccountType};
use reqwest::Error;

use super::{Proxy, web};

pub async fn check(account: &mut Account, proxy: Proxy) -> Result<(), Error> {

    let resp = web::get_request(format!("https://api.slothpixel.me/api/players/{}",account.uuid), proxy).await?;
    
    let rank = resp["rank"].to_string();
    let level = resp["level"].as_f64().unwrap_or_default();
    
    if rank != "null" {
        account.account_type = Some(AccountType::MinecraftHypixel{
            rank,
            level,
            last_login: resp["last_login"].as_i64().unwrap_or_default()
        });
    }

    Ok(())
}
