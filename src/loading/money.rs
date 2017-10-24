use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
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
        self.0 -= other.0;
    }
}

impl ops::AddAssign for Money {
    fn add_assign(&mut self, other: Money) {
        self.0 += other.0;
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

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.02}", self.to_f64())
    }
}

impl Money {
    pub fn to_f64(&self) -> f64 {
        (self.0 as f64) / 10000.0
    }

    pub fn zero() -> Money {
        Money(0)
    }
}

impl FromStr for Money {
    type Err = String;

    fn from_str(s: &str) -> Result<Money, String> {
        if s.starts_with("$") {
            Ok(Money(if let Ok(amt) = s[1..].parse::<f64>() {
                (amt * 10000.0) as i64
            } else {
                return Err(format!("unable to parse number '{}'", s));
            }))
        } else {
            Err(format!("'{}' does not look like money", s))
        }
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
        Ok(Money(value as i64 * 10000))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money(value as i64 * 10000))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money(value * 10000))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money((value * 100.0) as i64 * 100))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Money, E>
    where
        E: de::Error,
    {
        Ok(Money((value * 100.0) as i64 * 100))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Money, E>
    where
        E: de::Error,
    {
        let negative = v.starts_with("(") && v.ends_with(")");
        let v = if negative { &v[1..v.len() - 1] } else { v };
        let v = v.replace('$', "").replace(',', "");

        self.visit_f64(v.parse().map_err(|_| E::custom("Could not parse money"))?)
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
