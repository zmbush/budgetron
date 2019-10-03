// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_json;
use std::convert::TryInto;
use std::fmt;
use std::iter;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Copy, PartialEq, Clone, Eq, PartialOrd, Ord, Default)]
pub struct Money(i64);

impl ops::Sub<Money> for Money {
    type Output = Money;
    fn sub(self, other: Money) -> Money {
        Money(self.0 - other.0)
    }
}

impl ops::SubAssign for Money {
    fn sub_assign(&mut self, other: Money) {
        *self = *self - other;
    }
}

impl ops::AddAssign for Money {
    fn add_assign(&mut self, other: Money) {
        *self = *self + other;
    }
}

impl ops::MulAssign for Money {
    fn mul_assign(&mut self, other: Money) {
        *self = *self * other;
    }
}

impl ops::DivAssign for Money {
    fn div_assign(&mut self, other: Money) {
        *self = *self / other;
    }
}

impl iter::Sum for Money {
    fn sum<I>(iter: I) -> Money
    where
        I: Iterator<Item = Money>,
    {
        Money(iter.map(|m| m.0).sum())
    }
}

impl<'a> iter::Sum<&'a Money> for Money {
    fn sum<I>(iter: I) -> Money
    where
        I: Iterator<Item = &'a Money>,
    {
        Money(iter.map(|m| m.0).sum())
    }
}

impl ops::Neg for Money {
    type Output = Money;
    fn neg(mut self) -> Money {
        self.0 = -self.0;
        self
    }
}

impl ops::Add<Money> for Money {
    type Output = Money;
    fn add(self, other: Money) -> Money {
        Money(self.0 + other.0)
    }
}

impl ops::Mul<Money> for Money {
    type Output = Money;
    fn mul(self, other: Money) -> Money {
        Money(self.0 * other.0)
    }
}

impl ops::Mul<f64> for Money {
    type Output = Money;
    fn mul(self, other: f64) -> Money {
        Money::from_f64(self.to_f64() * other)
    }
}

impl ops::Mul<i32> for Money {
    type Output = Money;
    fn mul(self, other: i32) -> Money {
        self * f64::from(other)
    }
}

impl ops::Div<Money> for Money {
    type Output = Money;
    fn div(self, other: Money) -> Money {
        Money(self.0 / other.0)
    }
}

impl<'a> ops::Div<Money> for &'a Money {
    type Output = Money;
    fn div(self, other: Money) -> Money {
        Money(self.0 / other.0)
    }
}

impl Money {
    pub fn to_f64(&self) -> f64 {
        (self.0 as f64) / 10_000.0
    }

    pub fn from_f64(v: f64) -> Money {
        Money((v * 100.0) as i64 * 100)
    }

    pub fn from_i64(v: i64) -> Money {
        Money(v * 10_000)
    }

    pub fn abs(&self) -> Money {
        Money(self.0.abs())
    }

    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    pub fn zero() -> Money {
        Money(0)
    }

    pub fn uid(&self) -> String {
        format!("{:010}", self.0)
    }
}

impl FromStr for Money {
    type Err = String;

    fn from_str(s: &str) -> Result<Money, String> {
        serde_json::from_str(&format!("\"{}\"", s))
            .map_err(|_| format!("Unable to parse money {}", s))
    }
}

struct MoneyVisitor;
impl<'de> Visitor<'de> for MoneyVisitor {
    type Value = Money;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid money amount")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(i64::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(
            value.try_into().map_err(|_| E::custom("Too much money!"))?,
        ))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_f64(value))
    }

    fn visit_str<E>(self, v: &str) -> Result<Money, E>
    where
        E: de::Error,
    {
        let negative = v.starts_with('(') && v.ends_with(')');
        let v = if negative { &v[1..v.len() - 1] } else { v };
        let v = v.replace('$', "").replace(',', "");
        let mut parsed: f64 = v.parse().map_err(|_| E::custom("Could not parse money"))?;
        if negative {
            parsed = -parsed;
        }

        self.visit_f64(parsed)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Money, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MoneyVisitor)
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:.02}", self.to_f64()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn basic_operations() {
        assert_eq!(Money(10) + Money(10), Money(20));
        assert_eq!(Money(100) - Money(100), Money(0));
        assert_eq!(Money(1000) / Money(10), Money(100));
        assert_eq!(Money(10) * Money(5), Money(50));
    }

    #[test]
    fn assign_operations() {
        let mut mon = Money(10);
        mon += Money(30);
        assert_eq!(mon, Money(40));
        mon -= Money(12);
        assert_eq!(mon, Money(28));
        mon *= Money(2);
        assert_eq!(mon, Money(56));
        mon /= Money(2);
        assert_eq!(mon, Money(28));
    }

    #[test]
    fn iter_operations() {
        let monies = vec![Money(10), Money(5)];
        assert_eq!(monies.iter().sum::<Money>(), Money(15));
    }

    macro_rules! test_conversion {
        ($name:ident, $($s:expr => $o:expr),+) => {
            #[test]
            fn $name() {
                $(
                    let money_str = format!(r#"{}"#, $s);
                    let parsed_money: Money = serde_json::from_str(&money_str).unwrap();
                    assert_eq!(parsed_money, Money::from_f64($o));
                 )+
            }
        }
    }

    test_conversion!(
        parse_complex_numbers,
        r#""($100.0)""# => -100.0,
        r#""$100.0""# => 100.0,
        r#""-$100.0""# => -100.0
    );

    test_conversion!(
        parse_simpler_numbers,
        r#""$150.0""# => 150.0,
        r#""-125.50""# => -125.50
    );

    test_conversion!(
        plain_numbers,
        "100" => 100.0,
        "100.0" => 100.0,
        "0" => 0.0
    );
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.02}", self.to_f64())
    }
}
