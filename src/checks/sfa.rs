use crate::types::{Account, AccountType, CheckError, Proxy};

use super::{web, Check};
use async_trait::async_trait;

pub struct SFACheck;

#[async_trait]
impl Check for SFACheck {
    async fn check(self, account: &mut Account, proxy: &mut Proxy) -> Result<(), reqwest::Error> {
        let client = web::build_client(proxy)?;

        let resp = client
            .get("https://api.mojang.com/user/security/challenges")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", account.access_token.replace("\"", "")),
            )
            .send()
            .await?;

        let security_text = resp.text().await?;

        if security_text == "[]" {
            account.account_type = Some(AccountType::MinecraftSfa);
        }

        println!("{}", security_text);

        Ok(())
    }
}
