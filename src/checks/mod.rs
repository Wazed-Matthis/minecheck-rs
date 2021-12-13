use crate::types::*;
use async_trait::async_trait;
use hypixel::HypixelCheck;
use mojang::MojangCheck;
pub mod hypixel;
pub mod mojang;
pub mod sfa;
pub mod web;

pub async fn run_checks(account: &mut Account, proxy: &mut Proxy) -> Result<(), CheckError> {
    run_check(MojangCheck, account, &mut proxy.clone())
        .await
        .map_err(|_| CheckError::new("failed req...".to_owned()))?;

    run_check(HypixelCheck, account, &mut proxy.clone())
        .await
        .map_err(|_| CheckError::new("failed req...".to_owned()))?;

    run_check(sfa::SFACheck, account, &mut proxy.clone())
        .await
        .map_err(|_| CheckError::new("failed req...".to_owned()))?;

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
    async fn check(self, account: &mut Account, proxy: &mut Proxy) -> Result<(), reqwest::Error>;
}

pub async fn run_check(
    checker: impl Check,
    account: &mut Account,
    proxy: &mut Proxy,
) -> Result<(), reqwest::Error> {
    checker.check(account, proxy).await
}
