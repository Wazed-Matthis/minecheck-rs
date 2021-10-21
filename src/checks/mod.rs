use crate::types::*;

pub mod hypixel;
pub mod mojang;
pub mod web;

pub async fn run_checks(account: &mut Account, proxy: &mut Proxy) -> Result<(), CheckError> {
    match mojang::check(account, proxy.clone()).await {
        Ok(_) => {}
        Err(err) => {
            return Err(CheckError::new(String::from(format!(
                "failed request... {}",
                err
            ))));
        }
    };

    match hypixel::check(account, proxy.clone()).await {
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
