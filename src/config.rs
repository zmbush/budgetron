use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;

use email::Mailbox;
use lettre::email::ToMailbox;
use rustc_serialize::Decodable;
use toml;

#[derive(RustcDecodable)]
pub struct EmailAccount {
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(RustcDecodable)]
pub struct EmailDestination {
    pub name: String,
    pub email: String,
}

impl ToMailbox for EmailDestination {
    fn to_mailbox(&self) -> Mailbox {
        (&self).to_mailbox()
    }
}

impl<'a> ToMailbox for &'a EmailDestination {
    fn to_mailbox(&self) -> Mailbox {
        Mailbox::new_with_name(self.name.to_owned(), self.email.to_owned())
    }
}

#[derive(RustcDecodable)]
pub struct Email {
    pub to: Vec<EmailDestination>,
    pub from: EmailDestination,
    pub account: EmailAccount,
}

#[derive(RustcDecodable)]
pub struct SecureConfig {
    pub email: Option<Email>,
}

#[derive(RustcDecodable, Debug)]
pub struct Budgets {
    pub monthly: HashMap<String, f64>,
    pub quarterly: HashMap<String, f64>,
    pub yearly: HashMap<String, f64>,
}

#[derive(RustcDecodable, Debug)]
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

pub fn load_cfg<Cfg: Decodable>(fname: &str) -> Option<Cfg> {
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
            let path = env::home_dir()
                .unwrap_or(PathBuf::from("/"))
                .join(fname);
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

    toml::decode_str(&file_contents)
}
