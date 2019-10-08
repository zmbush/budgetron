// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::error::{BResult, BudgetError},
    serde::de::DeserializeOwned,
    serde::Deserialize,
    std::{collections::HashMap, env, fs::File, io::Read, path::PathBuf},
    toml,
};

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
pub struct CategoryConfig {
    pub categories: HashMap<String, Vec<String>>,
    pub ignored_accounts: Vec<String>,
}

impl CategoryConfig {
    pub fn find_category(&self, cat: &str) -> BResult<&str> {
        for (key, values) in &self.categories {
            if key == cat || (!values.is_empty() && values.contains(&cat.to_owned())) {
                return Ok(key);
            }
        }
        Err(BudgetError::NoCategoryFoundError(cat.to_owned()))
    }
}

pub fn load_cfg<Cfg>(fname: &str) -> BResult<Cfg>
where
    Cfg: DeserializeOwned,
{
    let config_contents = {
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
            let path = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("/"))
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

    Ok(toml::from_str(&config_contents)?)
}
