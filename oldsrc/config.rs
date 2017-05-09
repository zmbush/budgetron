

use email::Mailbox;
use lettre::email::IntoMailbox;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;

#[derive(Deserialize)]
pub struct EmailAccount {
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct EmailDestination {
    pub name: String,
    pub email: String,
}

impl IntoMailbox for EmailDestination {
    fn into_mailbox(self) -> Mailbox {
        (&self).into_mailbox()
    }
}

impl<'a> IntoMailbox for &'a EmailDestination {
    fn into_mailbox(self) -> Mailbox {
        Mailbox::new_with_name(self.name.to_owned(), self.email.to_owned())
    }
}

#[derive(Deserialize)]
pub struct Email {
    pub to: Vec<EmailDestination>,
    pub from: EmailDestination,
    pub account: EmailAccount,
}

#[derive(Deserialize)]
pub struct SecureConfig {
    pub email: Option<Email>,
}

#[derive(Deserialize, Debug)]
pub struct Budgets {
    pub monthly: HashMap<String, f64>,
    pub quarterly: HashMap<String, f64>,
    pub yearly: HashMap<String, f64>,
}

#[derive(Deserialize, Debug)]
pub struct CategoryConfig {
    pub categories: HashMap<String, Vec<String>>,
    pub budgets: Budgets,
    pub ignored_accounts: Vec<String>,
}

impl CategoryConfig {
    pub fn find_category(&self, cat: &str) -> Option<&str> {
        for (key, values) in &self.categories {
            if key == cat || (values.len() > 0 && values.contains(&cat.to_owned())) {
                return Some(key);
            }
        }
        println!("Unable to categorize transaction of category `{}`", cat);
        None
    }
}

pub fn load_cfg<Cfg: Deserialize>(fname: &str) -> Option<Cfg> {
    let file_contents = {
        if let Ok(mut dir) = env::current_dir() {
            let mut contents = "".to_owned();
            while dir.file_name() != None {
                if let Ok(mut f) = File::open(dir.join(fname)) {
                    let _ = f.read_to_string(&mut contents);
                    break;
                }
                dir.pop();
            }
            contents
        } else {
            let path = env::home_dir().unwrap_or(PathBuf::from("/")).join(fname);
            let ret: String = if let Ok(mut f) = File::open(path) {
                let mut s = String::new();
                let _ = f.read_to_string(&mut s);
                s
            } else {
                "".to_owned()
            };
            ret
        }
    };

    toml::from_str(&file_contents).ok()
}
