#[derive(Clone, Debug, Default)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub uuid: String,
    pub username: String,
    pub banned: bool,
    pub account_type: Option<AccountType>,
}

impl Account {
    pub fn new(combo: String) -> Self {
        Account {
            email: combo.split(':').next().unwrap().to_string(),
            password: combo.split(':').nth(1).unwrap().to_string(),
            ..Default::default()
        }
    }
}
#[derive(Clone, Debug)]
pub enum AccountType {
    MinecraftNfa,
    MinecraftSfa,
    MinecraftHypixel {
        level: f64,
        rank: String,
        last_login: i64,
    },
    Netflix {
        sub_type: String,
    },
    Steam {
        games: Vec<String>,
        level: i32,
        vac_bans: Vec<String>,
    },
    Uplay {
        games: Vec<String>,
    },
}

#[derive(Clone, Debug, Default)]
pub struct Proxy {
    pub combo: String,
    pub use_count: i32,
    pub proxy_type: ProxyType,
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

impl Default for ProxyType {
    fn default() -> Self {
        ProxyType::Http
    }
}

#[derive(Clone, Debug, Default)]
pub struct CheckError {
    pub reason: String,
}

impl CheckError {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}
