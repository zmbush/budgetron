
use self::Timeframe::*;
use chrono;
use chrono::Datelike;
use chrono::offset::TimeZone;
// use serde_json::value::{ToJson, Value};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops;

pub fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0) || ((year % 100 != 0) && (year % 400 == 0))
}


pub fn days_in_month(month: i64, year: i64) -> i64 {
    if month == 2 {
        28 + if is_leap_year(year) { 1 } else { 0 }
    } else {
        31 - (month - 1) % 7 % 2
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[allow(unused)]
pub enum Timeframe {
    Days(i64),
    Weeks(i64),
    Months(i64),
    Quarters(i64),
    Years(i64),
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (d, name, v) = match *self {
            Days(num) => ("d", "Day", num),
            Weeks(num) => ("w", "Week", num),
            Months(num) => ("m", "Month", num),
            Quarters(num) => ("q", "Quarter", num),
            Years(num) => ("y", "Year", num),
        };

        if f.alternate() {
            write!(f, "{}{}", v, d)
        } else {
            if v == 1 {
                write!(f, "{}", name)
            } else {
                write!(f, "{} {}s", v, name)
            }
        }
    }
}

impl Timeframe {
    fn invert(&self) -> Timeframe {
        match *self {
            Days(n) => Days(-n),
            Weeks(n) => Weeks(-n),
            Months(n) => Months(-n),
            Quarters(n) => Quarters(-n),
            Years(n) => Years(-n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Date {
    pub date: chrono::Date<chrono::UTC>,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let y = self.date.year();
        let m = self.date.month();
        let d = self.date.day();

        if f.alternate() {
            write!(f, "{:04}{:02}{:02}", y, m, d)
        } else {
            write!(f, "{}/{}/{}", m, d, y)
        }
    }
}

impl Date {
    fn move_days(&mut self, days: i64) {
        self.date = self.date + chrono::Duration::days(days);
    }

    fn move_one_month(&mut self, forward: bool) {
        let days = days_in_month(self.date.month() as i64 - if forward { 0 } else { 1 },
                                 self.date.year() as i64);
        self.move_days(days * if forward { 1 } else { -1 });
    }

    fn move_months(&mut self, months: i64) {
        let forward = months >= 0;
        let months = months.abs();
        for _ in 0..months {
            self.move_one_month(forward);
        }
    }

    fn add_tf(&mut self, time_frame: &Timeframe) {
        match *time_frame {
            Weeks(num) => self.move_days(7 * num),
            Days(num) => self.move_days(num),
            Months(num) => self.move_months(num),
            Quarters(num) => self.move_months(3 * num),
            Years(num) => self.move_months(12 * num),
        }
    }

    pub fn align_to_month(&mut self) {
        let days = self.date.day0() as i64;
        *self -= Days(days);
    }


    pub fn align_to_quarter(&mut self) {
        self.align_to_month();
        let months = self.date.month0() as i64 % 3;
        *self -= Months(months);
    }

    pub fn day_of_week(&self) -> i64 {
        self.date.weekday().num_days_from_monday() as i64
    }

    pub fn align_to_week(&mut self) {
        while self.day_of_week() != 0 {
            *self -= Days(1);
        }
    }

    pub fn align_to_year(&mut self) {
        *self = Date::ymd(self.year(), 1, 1);
    }

    pub fn ymd(y: i32, m: i32, d: i32) -> Date {
        Date { date: chrono::UTC.ymd(y, m as u32, d as u32) }
    }

    pub fn year(&self) -> i32 {
        self.date.year() as i32
    }
}

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> ops::$imp<$u> for &'a $t {
            type Output = <$t as ops::$imp<$u>>::Output;

            fn $method(self, other: $u) -> <$t as ops::$imp<$u>>::Output {
                ops::$imp::$method(*self, other)
            }
        }

        impl<'a> ops::$imp<&'a $u> for $t {
            type Output = <$t as ops::$imp<$u>>::Output;

            fn $method(self, other: &'a $u) -> <$t as ops::$imp<$u>>::Output {
                ops::$imp::$method(self, *other)
            }
        }

        impl<'a, 'b> ops::$imp<&'a $u> for &'b $t {
            type Output = <$t as ops::$imp<$u>>::Output;

            fn $method(self, other: &'a $u) -> <$t as ops::$imp<$u>>::Output {
                ops::$imp::$method(*self, *other)
            }
        }
    }
}

impl ops::Sub<Timeframe> for Date {
    type Output = Date;

    fn sub(mut self, other: Timeframe) -> Date {
        self.add_tf(&other.invert());

        self
    }
}
forward_ref_binop!(impl Sub, sub for Date, Timeframe);

impl ops::SubAssign<Timeframe> for Date {
    fn sub_assign(&mut self, other: Timeframe) {
        self.add_tf(&other.invert());
    }
}

impl<'a> ops::SubAssign<&'a Timeframe> for Date {
    fn sub_assign(&mut self, other: &Timeframe) {
        self.add_tf(&other.invert());
    }
}

impl ops::Add<Timeframe> for Date {
    type Output = Date;

    fn add(mut self, other: Timeframe) -> Date {
        self.add_tf(&other);

        self
    }
}
forward_ref_binop!(impl Add, add for Date, Timeframe);

impl ops::AddAssign<Timeframe> for Date {
    fn add_assign(&mut self, other: Timeframe) {
        self.add_tf(&other);
    }
}
impl<'a> ops::AddAssign<&'a Timeframe> for Date {
    fn add_assign(&mut self, other: &Timeframe) {
        self.add_tf(other);
    }
}

impl ops::Div<Timeframe> for Timeframe {
    type Output = f64;

    fn div(self, other: Timeframe) -> f64 {
        fn numerize(tf: Timeframe) -> f64 {
            let ret = match tf {
                Days(n) => n,
                Weeks(n) => 7 * n,
                Months(n) => 30 * n,
                Quarters(n) => 30 * 3 * n,
                Years(n) => 30 * 12 * n,
            };

            ret as f64
        }

        let numer = numerize(self);
        let denom = numerize(other);

        numer / denom
    }
}
forward_ref_binop!(impl Div, div for Timeframe, Timeframe);

struct DateVisitor;
impl de::Visitor for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid date")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Date, E> {
        let error = Err(E::custom(&format!("Bad date format '{}'", value)));

        macro_rules! get_num {
            ($d:ident) => {
                match $d.next() {
                    Some(s) => match s.parse() {
                        Ok(i) => i,
                        Err(_) => return error
                    },
                    None => return error
                }
            }
        }

        let mut parts = value.split("/");
        let m = get_num!(parts);
        let d = get_num!(parts);
        let y = get_num!(parts);
        Ok(Date::ymd(y, m, d))
    }
}

impl Deserialize for Date {
    fn deserialize<D: Deserializer>(d: D) -> Result<Self, D::Error> {
        d.deserialize_str(DateVisitor)
    }
}

impl Decodable for Date {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let s = try!(d.read_str());
        let error = Err(d.error(&format!("Bad date format '{}'", s)));

        macro_rules! get_num {
            ($d:ident) => {
                match $d.next() {
                    Some(s) => match s.parse() {
                        Ok(i) => i,
                        Err(_) => return error
                    },
                    None => return error
                }
            }
        }

        let mut parts = s.split("/");
        let m = get_num!(parts);
        let d = get_num!(parts);
        let y = get_num!(parts);
        Ok(Date::ymd(y, m, d))
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{}", self))
    }
}

impl Encodable for Date {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(&format!("{}", self))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Timeframe::*;

    #[test]
    fn test_date() {
        let mut d = Date::ymd(2016, 1, 1);
        assert_eq!(d, Date::ymd(2016, 1, 1));

        d += &Months(1);
        assert_eq!(d, Date::ymd(2016, 2, 1));

        d += &Months(1);
        assert_eq!(d, Date::ymd(2016, 3, 1));

        d.align_to_week();
        assert_eq!(d, Date::ymd(2016, 2, 29));

        d.align_to_month();
        assert_eq!(d, Date::ymd(2016, 2, 1));

        d.align_to_quarter();
        assert_eq!(d, Date::ymd(2016, 1, 1));

        d += &Quarters(1);
        assert_eq!(d, Date::ymd(2016, 4, 1));

        d = d + &Months(1);
        assert_eq!(d, Date::ymd(2016, 5, 1));

        d += &Quarters(1);
        assert_eq!(d, Date::ymd(2016, 8, 1));

        d.align_to_quarter();
        assert_eq!(d, Date::ymd(2016, 7, 1));
    }

    #[test]
    fn test_leap_year() {
        assert!(!is_leap_year(2015));
        assert!(is_leap_year(2016));
    }

    #[test]
    fn test_days_in_month() {
        macro_rules! tests {
            ($(year $y:expr {
                $($m:expr => $count:expr),*
            }),*) => {{
                $($(assert!(days_in_month($m, $y) == $count));*);*
            }}
        }
        tests! {
            year 2016 {
                1 => 31,
                2 => 29,
                3 => 31,
                4 => 30,
                5 => 31,
                6 => 30,
                7 => 31,
                8 => 31,
                9 => 30,
                10 => 31,
                11 => 30,
                12 => 31
            },
            year 2015 {
                1 => 31,
                2 => 28,
                3 => 31,
                4 => 30,
                5 => 31,
                6 => 30,
                7 => 31,
                8 => 31,
                9 => 30,
                10 => 31,
                11 => 30,
                12 => 31
            }
        }
    }
}
