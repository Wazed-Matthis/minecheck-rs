use crate::types::*;
use async_trait::async_trait;
use hypixel::HypixelCheck;
use mojang::MojangCheck;
pub mod hypixel;
pub mod mojang;
pub mod sfa;
pub mod web;

pub async fn run_checks(account: &mut Account, proxy: &mut Proxy) -> Result<(), CheckError> {
    match run_check(MojangCheck, account, &mut proxy.clone()).await {
        Ok(_) => {}
        Err(err) => {
            return Err(CheckError::new(String::from(format!(
                "failed request... {}",
                err
            ))));
        }
    };

    match run_check(HypixelCheck, account, &mut proxy.clone()).await {
        Ok(_) => {}
        Err(err) => {
            return Err(CheckError::new(String::from(format!(
                "failed request... {}",
                err
            ))));
        }
    };

    match run_check(sfa::SFACheck, account, &mut proxy.clone()).await {
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
    async fn check(self, account: &mut Account, proxy: &mut Proxy) -> Result<(), reqwest::Error>;
}

pub async fn run_check<T>(
    checker: T,
    account: &mut Account,
    proxy: &mut Proxy,
) -> Result<(), reqwest::Error>
where
    T: Check,
{
    checker.check(account, proxy).await
}
