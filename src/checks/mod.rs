use crate::types::Account;

pub mod hypixel;
pub mod mojang;
pub mod web;

#[derive(Clone, Debug)]
pub struct Proxy {
    combo: String,
    use_count: i32,
    proxy_type: ProxyType,
}

impl Proxy {
    pub fn new(combo: String, use_count: i32, proxy_type: ProxyType) -> Self {
        Self {
            combo,
            use_count,
            proxy_type,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ProxyType {
    Http,
    Https { username: String, password: String },
    Socks4,
    Socks5,
}

pub async fn run_checks(account: &mut Account, proxy: &mut Proxy) -> Result<(), CheckError> {
    
    match mojang::check(account, proxy.clone()).await {
        Ok(_) => {},
        Err(err) => {
            return Err(CheckError::new(String::from(format!("failed request... {}", err))));
        },
    };

    match hypixel::check(account, proxy.clone()).await {
        Ok(_) => {},
        Err(err) => {
            return Err(CheckError::new(String::from(format!("failed request... {}", err))));
        },
    };

    if account.banned {
        proxy.use_count+=1;
    }

    if proxy.use_count >= 3 {
        return Err(CheckError::new(String::from("Bad proxy!")));
    }
    Ok(())
}

pub struct CheckError{
    pub reason: String
}

impl CheckError {
    pub fn new(reason: String) -> Self { Self { reason } }
}
