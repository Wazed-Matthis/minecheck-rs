use crate::types::*;
use async_trait::async_trait;
use mojang::MojangCheck;
use hypixel::HypixelCheck;
pub mod hypixel;
pub mod mojang;
pub mod web;

pub async fn run_checks(account: &mut Account, proxy: &mut Proxy) -> Result<(), CheckError> {
    match run_check(MojangCheck, account, proxy).await {
        Ok(_) => {}
        Err(err) => {
            return Err(CheckError::new(String::from(format!(
                "failed request... {}",
                err
            ))));
        }
    };

    match run_check(HypixelCheck, account, proxy.clone()).await {
        Ok(_) => {}
        Err(err) => {
            return Err(CheckError::new(String::from(format!(
                "failed request... {}",
                err
            ))));
        }
    };

    if account.banned {
        proxy.use_count += 1;
    }

    if proxy.use_count >= 3 {
        return Err(CheckError::new(String::from("Bad proxy!")));
    }
    Ok(())
}

#[async_trait]
pub trait Check {
    async fn check(self, account: &mut Account, proxy: Proxy) -> Result<(), reqwest::Error>;
}

pub async fn run_check<T>(checker: T, account: &mut Account, proxy: Proxy) -> Result<(), reqwest::Error>
where
    T: Check,
{
    checker.check(account, proxy).await
}
