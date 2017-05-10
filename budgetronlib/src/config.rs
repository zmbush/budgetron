use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use error::{BResult, BudgetError};

#[derive(Deserialize)]
pub struct EmailAccount {
    pub username: String,
    pub password: String,
    pub server: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct SecureConfig {}

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
    pub fn find_category(&self, cat: &str) -> BResult<&str> {
        for (key, values) in &self.categories {
            if key == cat || (values.len() > 0 && values.contains(&cat.to_owned())) {
                return Ok(key);
            }
        }
        Err(BudgetError::NoCategoryFoundError(cat.to_owned()))
    }
}

pub fn open_cfg(fname: &str) -> String {
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
}
