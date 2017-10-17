use regex;
use serde::de::{self, Visitor, Deserialize, Deserializer};
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
            Err(e) => Err(E::custom(
                format!("Unable to parse `{}` as regex {}", value, e),
            )),
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
