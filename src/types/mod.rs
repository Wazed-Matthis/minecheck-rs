
#[derive(Clone,Debug)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub uuid: String,
    pub username: String,
    pub banned: bool,
    pub account_type: Option<AccountType>
}

impl Account {
    pub fn new(combo: String) -> Self {
        Account {
            email: combo.split(':').next().unwrap().to_string(),
            password: combo.split(':').nth(1).unwrap().to_string(),
            access_token: String::new(),
            uuid: String::new(),
            username: String::new(),
            banned: false,
            account_type: None 
        }
    }
}
#[derive(Clone, Debug)]
pub enum AccountType{
    MinecraftNfa,
    MinecraftHypixel{
        level: f64,
        rank: String,
        last_login: i64
    },
    Netflix{
        sub_type: String,
    },
    Steam{
        games: Vec<String>,
        level: i32,
        vac_bans: Vec<String>
    },
    Uplay{
        games: Vec<String>
    }
}