// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use serde_json;
use std::ops;
use std::iter;

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
        (self.0 as f64) / 10000.0
    }

    pub fn from_f64(v: f64) -> Money {
        Money((v * 100.0) as i64 * 100)
    }

    pub fn from_i64(v: i64) -> Money {
        Money(v * 10000)
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

    fn visit_i8<E>(self, value: i8) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(value as i64))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(value as i64))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_i64(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_f64(value as f64))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money::from_f64(value))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Money, E>
    where
        E: de::Error,
    {
        let negative = v.starts_with("(") && v.ends_with(")");
        let v = if negative { &v[1..v.len() - 1] } else { v };
        let v = v.replace('$', "").replace(',', "");
        let mut parsed: f64 = v.parse().map_err(|_| E::custom("Could not parse money"))?;
        if negative {
            parsed = -parsed;
        }

        self.visit_f64(parsed)
    }

    fn visit_str<E>(self, v: &str) -> Result<Money, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Money, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(&v)
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Money, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MoneyVisitor)
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
                    let money_str = format!(r#""{}""#, $s);
                    let parsed_money: Money = serde_json::from_str(&money_str).unwrap();
                    assert_eq!(parsed_money, Money::from_f64($o));
                )+
            }
        }
    }

    test_conversion!(
        parse_complex_numbers,
        "($100.0)" => -100.0,
        "$100.0" => 100.0,
        "-$100.0" => -100.0
    );

    test_conversion!(
        parse_simpler_numbers,
        "$150.0" => 150.0,
        "-125.50" => -125.50
    );
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.02}", self.to_f64())
    }
}
