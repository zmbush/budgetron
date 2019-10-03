// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use regex::{self, Captures};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug)]
pub struct Regex(pub regex::Regex);

struct RegexVisitor;
impl<'de> Visitor<'de> for RegexVisitor {
    type Value = Regex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid regex string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Regex, E>
    where
        E: de::Error,
    {
        match regex::Regex::new(value) {
            Ok(re) => Ok(Regex(re)),
            Err(e) => Err(E::custom(format!(
                "Unable to parse `{}` as regex {}",
                value, e
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(deserializer: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RegexVisitor)
    }
}

impl Regex {
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        self.0.captures(text)
    }
}
