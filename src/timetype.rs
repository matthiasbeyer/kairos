//! The module for the TimeType
//!

use chrono::NaiveDateTime;
use chrono::NaiveDate;
use chrono::Datelike;
use chrono::Timelike;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use result::Result;
use error::KairosErrorKind as KEK;
use error::KairosError as KE;
use error_chain::ChainedError;

/// A Type of Time, currently based on chrono::NaiveDateTime
#[derive(Debug, Clone)]
pub enum TimeType {
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Months(i64),
    Years(i64),

    Moment(NaiveDateTime),

    Addition(Box<TimeType>, Box<TimeType>),
    Subtraction(Box<TimeType>, Box<TimeType>),
}

impl Add for TimeType {
    type Output = TimeType;

    fn add(self, rhs: TimeType) -> Self::Output {
        TimeType::Addition(Box::new(self), Box::new(rhs))
    }
}

impl AddAssign for TimeType {
    fn add_assign(&mut self, rhs: TimeType) {
        *self = TimeType::Addition(Box::new(self.clone()), Box::new(rhs));
    }
}

impl Sub for TimeType {
    type Output = TimeType;

    fn sub(self, rhs: TimeType) -> Self::Output {
        TimeType::Subtraction(Box::new(self), Box::new(rhs))
    }
}

impl SubAssign for TimeType {
    fn sub_assign(&mut self, rhs: TimeType) {
        *self = TimeType::Subtraction(Box::new(self.clone()), Box::new(rhs));
    }
}

/// The TimeType type
///
/// # Warning
///
/// If the TimeType is _larger_ than the queried type (E.G. querying a "minutes" on a "month"),
/// the following rules are applied:
///
/// * 60 Seconds make a Minute
/// * 60 Minutes make a Hour
/// * 24 Hours make a Day
/// * 7 Days make a Week
/// * 4 Weeks make a Month
/// * 12 Months make a Year
///
/// Whether these may be correct or not in the current year. The return value of the function
/// is calculated appropriately. So, calling the `get_seconds()` function on 5 minutes returns
/// `60 * 5`.
///
/// If the TimeType is _smaller_ than the queried type (E.G. querying a "month" on a
/// "minutes"), zero is returned.
///
/// Also, if the TimeType is "5 weeks", querying a month returns `1`, as 5 weeks contain one
/// full month.
///
impl TimeType {

    pub fn is_a_amount(&self) -> bool {
        match *self {
            TimeType::Seconds(_) |
            TimeType::Minutes(_) |
            TimeType::Hours(_)   |
            TimeType::Days(_)    |
            TimeType::Months(_)  |
            TimeType::Years(_)   => true,
            _                    => false,
        }
    }

    pub fn is_moment(&self) -> bool {
        match *self {
            TimeType::Moment(_) => true,
            _                   => false,
        }
    }

    pub fn is_addition(&self) -> bool {
        match *self {
            TimeType::Addition(_, _) => true,
            _                        => false,
        }
    }

    pub fn is_subtraction(&self) -> bool {
        match *self {
            TimeType::Subtraction(_, _) => true,
            _                           => false,
        }
    }

    pub fn seconds(i: i64) -> TimeType {
        TimeType::Seconds(i)
    }

    pub fn minutes(i: i64) -> TimeType {
        TimeType::Minutes(i)
    }

    pub fn hours(i: i64) -> TimeType {
        TimeType::Hours(i)
    }

    pub fn days(i: i64) -> TimeType {
        TimeType::Days(i)
    }

    /// Helper for `TimeType::days(i * 7)`
    pub fn weeks(i: i64) -> TimeType {
        TimeType::Days(i * 7)
    }

    pub fn months(i: i64) -> TimeType {
        TimeType::Months(i)
    }

    pub fn years(i: i64) -> TimeType {
        TimeType::Years(i)
    }

    pub fn moment(ndt: NaiveDateTime) -> TimeType {
        TimeType::Moment(ndt)
    }

    /// Get the number of seconds, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_seconds(&self) -> i64 {
        match *self {
            TimeType::Seconds(d) => d,
            TimeType::Minutes(d) => d * 60,
            TimeType::Hours(d)   => d * 60 * 60,
            TimeType::Days(d)    => d * 60 * 60 * 24,
            TimeType::Months(d)  => d * 60 * 60 * 24 * 30,
            TimeType::Years(d)   => d * 60 * 60 * 24 * 30 * 12,
            _                    => 0
        }
    }

    /// Get the number of minutes, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_minutes(&self) -> i64 {
        match *self {
            TimeType::Seconds(s) => s / 60,
            TimeType::Minutes(d) => d,
            TimeType::Hours(d)   => d * 60,
            TimeType::Days(d)    => d * 60 * 24,
            TimeType::Months(d)  => d * 60 * 24 * 30,
            TimeType::Years(d)   => d * 60 * 24 * 30 * 12,
            _ => 0
        }
    }

    /// Get the number of hours, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_hours(&self) -> i64 {
        match *self {
            TimeType::Seconds(s) => s / 60 / 60,
            TimeType::Minutes(d) => d / 60,
            TimeType::Hours(d)   => d,
            TimeType::Days(d)    => d * 24,
            TimeType::Months(d)  => d * 24 * 30,
            TimeType::Years(d)   => d * 24 * 30 * 12,
            _ => 0
        }
    }

    /// Get the number of days, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_days(&self) -> i64 {
        match *self {
            TimeType::Seconds(s) => s / 24 / 60 / 60,
            TimeType::Minutes(d) => d / 24 / 60,
            TimeType::Hours(d)   => d / 24,
            TimeType::Days(d)    => d,
            TimeType::Months(d)  => d * 30,
            TimeType::Years(d)   => d * 30 * 12,
            _ => 0
        }
    }

    /// Get the number of months, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_months(&self) -> i64 {
        match *self {
            TimeType::Seconds(s) => s / 30 / 24 / 60 / 60,
            TimeType::Minutes(d) => d / 30 / 24 / 60,
            TimeType::Hours(d)   => d / 30 / 24,
            TimeType::Days(d)    => d / 30,
            TimeType::Months(d)  => d,
            TimeType::Years(d)   => d * 12,
            _ => 0
        }
    }

    /// Get the number of years, if the TimeType is not a duration type, zero is returned
    ///
    /// # Warning
    ///
    /// If the type is actually a smaller one (eg. calling get_minutes() on a seconds instance) the
    /// following rules are applied:
    ///
    /// * A minute is 60 seconds
    /// * A hour is 60 minutes
    /// * A day is 24 hours
    /// * A month is 30 days
    /// * A year is 12 months
    ///
    /// Which might not be always correct.
    pub fn get_years(&self) -> i64 {
        match *self {
            TimeType::Seconds(s) => s / 12 / 30 / 24 / 60 / 60,
            TimeType::Minutes(d) => d / 12 / 30 / 24 / 60,
            TimeType::Hours(d)   => d / 12 / 30 / 24,
            TimeType::Days(d)    => d / 12 / 30,
            TimeType::Months(d)  => d / 12,
            TimeType::Years(d)   => d,
            _ => 0
        }
    }

    pub fn get_moment(&self) -> Option<&NaiveDateTime> {
        match *self {
            TimeType::Moment(ref m) => Some(&m),
            _                   => None,
        }
    }

    pub fn calculate(self) -> Result<TimeType> {
        do_calculate(self)
    }
}

fn do_calculate(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Addition(a, b)    => add(a, b),
        TT::Subtraction(a, b) => sub(a, b),
        x                     => Ok(x)
    }
}

fn add(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Moment(mom), thing) => add_to_moment(mom, thing),
        (thing, TT::Moment(mom)) => Err(KE::from_kind(KEK::CannotAdd(thing, TT::Moment(mom)))),

        (TT::Seconds(a), other) => add_to_seconds(a, other),
        (other, TT::Seconds(a)) => add_to_seconds(a, other),

        (TT::Minutes(a), other) => add_to_minutes(a, other),
        (other, TT::Minutes(a)) => add_to_minutes(a, other),

        (TT::Hours(a), other)   => add_to_hours(a, other),
        (other, TT::Hours(a))   => add_to_hours(a, other),

        (TT::Days(a), other)    => add_to_days(a, other),
        (other, TT::Days(a))    => add_to_days(a, other),

        (TT::Months(a), other)  => add_to_months(a, other),
        (other, TT::Months(a))  => add_to_months(a, other),

        (TT::Years(a), other)   => add_to_years(a, other),
        (other, TT::Years(a))   => add_to_years(a, other),

        (TT::Addition(a, b), other)      => add(a, b)
            .map(Box::new)
            .and_then(|bx| add(bx, Box::new(other))),
        (other, TT::Addition(a, b))      => add(a, b)
            .map(Box::new)
            .and_then(|bx| add(Box::new(other), bx)),
        (TT::Subtraction(a, b), other) => sub(a, b)
            .map(Box::new)
            .and_then(|bx| add(Box::new(other), bx)),
        (other, TT::Subtraction(a, b)) => do_calculate(*a)
            .map(Box::new)
            .and_then(|bx| add(Box::new(other), bx))
            .and_then(|rx| sub(Box::new(rx), b)),
        others                           => unimplemented!(),
    }
}

fn add_to_seconds(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount)),
        TT::Minutes(a)        => Ok(TT::Seconds(a * 60 + amount)),
        TT::Hours(a)          => Ok(TT::Seconds(a * 60 * 60 + amount)),
        TT::Days(a)           => Ok(TT::Seconds(a * 60 * 60 * 24 + amount)),
        TT::Months(a)         => Ok(TT::Seconds(a * 60 * 60 * 24 * 30 + amount)),
        TT::Years(a)          => Ok(TT::Seconds(a * 60 * 60 * 24 * 30 * 12 + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Seconds(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_seconds(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_seconds(amount, try!(sub(b, c))),
    }
}

fn add_to_minutes(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount * 60)),
        TT::Minutes(a)        => Ok(TT::Minutes(a + amount)),
        TT::Hours(a)          => Ok(TT::Minutes(a * 60 + amount)),
        TT::Days(a)           => Ok(TT::Minutes(a * 60 * 24 + amount)),
        TT::Months(a)         => Ok(TT::Minutes(a * 60 * 24 * 30 + amount)),
        TT::Years(a)          => Ok(TT::Minutes(a * 60 * 24 * 30 * 12 + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Minutes(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_minutes(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_minutes(amount, try!(sub(b, c))),
    }
}

fn add_to_hours(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount * 60 * 60)),
        TT::Minutes(a)        => Ok(TT::Minutes(a + amount * 60)),
        TT::Hours(a)          => Ok(TT::Hours(  a + amount)),
        TT::Days(a)           => Ok(TT::Hours(  a * 24 + amount)),
        TT::Months(a)         => Ok(TT::Hours(  a * 24 * 30 + amount)),
        TT::Years(a)          => Ok(TT::Hours(  a * 24 * 30 * 12 + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Hours(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_hours(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_hours(amount, try!(sub(b, c))),
    }
}

fn add_to_days(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount * 24 * 60 * 60)),
        TT::Minutes(a)        => Ok(TT::Minutes(a + amount * 24 * 60)),
        TT::Hours(a)          => Ok(TT::Hours(  a + amount * 24)),
        TT::Days(a)           => Ok(TT::Days(   a + amount)),
        TT::Months(a)         => Ok(TT::Days(   a * 30 + amount)),
        TT::Years(a)          => Ok(TT::Days(   a * 30 * 12 + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Days(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_days(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_days(amount, try!(sub(b, c))),
    }
}

fn add_to_months(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount * 30 * 24 * 60 * 60)),
        TT::Minutes(a)        => Ok(TT::Minutes(a + amount * 30 * 24 * 60)),
        TT::Hours(a)          => Ok(TT::Hours(  a + amount * 30 * 24)),
        TT::Days(a)           => Ok(TT::Days(   a + amount * 30)),
        TT::Months(a)         => Ok(TT::Months( a + amount)),
        TT::Years(a)          => Ok(TT::Months( a * 12 + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Months(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_months(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_months(amount, try!(sub(b, c))),
    }
}

fn add_to_years(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(a + amount * 12 * 30 * 24 * 60 * 60)),
        TT::Minutes(a)        => Ok(TT::Minutes(a + amount * 12 * 30 * 24 * 60)),
        TT::Hours(a)          => Ok(TT::Hours(  a + amount * 12 * 30 * 24)),
        TT::Days(a)           => Ok(TT::Days(   a + amount * 12 * 30)),
        TT::Months(a)         => Ok(TT::Months( a + amount * 12)),
        TT::Years(a)          => Ok(TT::Years(  a + amount)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Years(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => add_to_years(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => add_to_years(amount, try!(sub(b, c))),
    }
}

#[inline]
fn adjust_times_add(mut y: i64, mut mo: i64, mut d: i64, mut h: i64, mut mi: i64, mut s: i64)
    -> (i64, i64, i64, i64, i64, i64)
{
    macro_rules! fix {
        {
            $base:ident,
            $border:expr,
            $next:ident
        } => {
            while $base >= $border {
                $next += 1;
                $base -= $border;
            }
        }
    }

    fix! { s , 60, mi }
    fix! { mi, 60, h  }
    fix! { h , 24, d  }

    if mo == 1 || mo == 3 || mo == 5 || mo == 7 || mo == 8 || mo == 10 || mo == 12 {
        fix! { d , 31, mo }
    } else {
        fix! { d , 30, mo }
    }

    fix! { mo, 12, y  }

    (y, mo, d, h, mi, s)
}

fn add_to_moment(mom: NaiveDateTime, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64 + a;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Minutes(a)        => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64 + a;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Hours(a)          => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64 + a;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Days(a)           => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64 + a;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Months(a)         => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64 + a;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Years(a)          => {
            let y  = mom.year() as i64 + a;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_add(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Moment(mom), TT::Moment(m)))),
        TT::Addition(a, b)    => add_to_moment(mom, try!(add(a, b))),
        TT::Subtraction(a, b) => add_to_moment(mom, try!(sub(a, b))),
    }
}

fn sub(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Moment(mom), thing) => sub_from_moment(mom, thing),
        (thing, TT::Moment(mom)) => Err(KE::from_kind(KEK::CannotSub(thing, TT::Moment(mom)))),

        (TT::Seconds(a), other) => sub_from_seconds(a, other),
        (other, TT::Seconds(a)) => sub_from_seconds(a, other),

        (TT::Minutes(a), other) => sub_from_minutes(a, other),
        (other, TT::Minutes(a)) => sub_from_minutes(a, other),

        (TT::Hours(a), other)   => sub_from_hours(a, other),
        (other, TT::Hours(a))   => sub_from_hours(a, other),

        (TT::Days(a), other)    => sub_from_days(a, other),
        (other, TT::Days(a))    => sub_from_days(a, other),

        (TT::Months(a), other)  => sub_from_months(a, other),
        (other, TT::Months(a))  => sub_from_months(a, other),

        (TT::Years(a), other)   => sub_from_years(a, other),
        (other, TT::Years(a))   => sub_from_years(a, other),

        (TT::Subtraction(a, b), other)   => sub(a, b)
            .map(Box::new)
            .and_then(|bx| sub(bx, Box::new(other))),
        (other, TT::Subtraction(a, b))   => sub(a, b)
            .map(Box::new)
            .and_then(|bx| sub(Box::new(other), bx)),
        (TT::Addition(a, b), other) => add(a, b)
            .map(Box::new)
            .and_then(|bx| sub(bx, Box::new(other))),
        (other, TT::Addition(a, b)) => do_calculate(*a)
            .map(Box::new)
            .and_then(|bx| sub(Box::new(other), bx))
            .and_then(|rx| add(Box::new(rx), b)),
        others                           => unimplemented!(),
    }
}

fn sub_from_seconds(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount - a)),
        TT::Minutes(a)        => Ok(TT::Seconds(amount - a * 60)),
        TT::Hours(a)          => Ok(TT::Seconds(amount - a * 60 * 60)),
        TT::Days(a)           => Ok(TT::Seconds(amount - a * 60 * 60 * 24)),
        TT::Months(a)         => Ok(TT::Seconds(amount - a * 60 * 60 * 24 * 30)),
        TT::Years(a)          => Ok(TT::Seconds(amount - a * 60 * 60 * 24 * 30 * 12)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Seconds(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_seconds(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_seconds(amount, try!(sub(b, c))),
    }
}

fn sub_from_minutes(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount * 60 - a)),
        TT::Minutes(a)        => Ok(TT::Minutes(amount - a)),
        TT::Hours(a)          => Ok(TT::Minutes(amount - a * 60)),
        TT::Days(a)           => Ok(TT::Minutes(amount - a * 60 * 24)),
        TT::Months(a)         => Ok(TT::Minutes(amount - a * 60 * 24 * 30)),
        TT::Years(a)          => Ok(TT::Minutes(amount - a * 60 * 24 * 30 * 12)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Minutes(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_minutes(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_minutes(amount, try!(sub(b, c))),
    }
}

fn sub_from_hours(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount * 60 * 60 - a)),
        TT::Minutes(a)        => Ok(TT::Minutes(amount * 60 - a)),
        TT::Hours(a)          => Ok(TT::Hours(amount -   a)),
        TT::Days(a)           => Ok(TT::Hours(amount -   a * 24)),
        TT::Months(a)         => Ok(TT::Hours(amount -   a * 24 * 30)),
        TT::Years(a)          => Ok(TT::Hours(amount -   a * 24 * 30 * 12)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Hours(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_hours(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_hours(amount, try!(sub(b, c))),
    }
}

fn sub_from_days(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount * 24 * 60 * 60 - a)),
        TT::Minutes(a)        => Ok(TT::Minutes(amount * 24 * 60 - a)),
        TT::Hours(a)          => Ok(TT::Hours(amount * 24 -   a)),
        TT::Days(a)           => Ok(TT::Days(amount -    a)),
        TT::Months(a)         => Ok(TT::Days(amount -    a * 30)),
        TT::Years(a)          => Ok(TT::Days(amount -    a * 30 * 12)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Days(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_days(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_days(amount, try!(sub(b, c))),
    }
}

fn sub_from_months(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount * 30 * 24 * 60 * 60 - a)),
        TT::Minutes(a)        => Ok(TT::Minutes(amount * 30 * 24 * 60 - a)),
        TT::Hours(a)          => Ok(TT::Hours(amount * 30 * 24 -   a)),
        TT::Days(a)           => Ok(TT::Days(amount * 30 -    a)),
        TT::Months(a)         => Ok(TT::Months(amount -  a)),
        TT::Years(a)          => Ok(TT::Months(amount -  a * 12)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Months(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_months(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_months(amount, try!(sub(b, c))),
    }
}

fn sub_from_years(amount: i64, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => Ok(TT::Seconds(amount * 12 * 30 * 24 * 60 * 60 - a)),
        TT::Minutes(a)        => Ok(TT::Minutes(amount * 12 * 30 * 24 * 60 - a)),
        TT::Hours(a)          => Ok(TT::Hours(amount * 12 * 30 * 24 -   a)),
        TT::Days(a)           => Ok(TT::Days(amount * 12 * 30 -    a)),
        TT::Months(a)         => Ok(TT::Months(amount * 12 -  a)),
        TT::Years(a)          => Ok(TT::Years(amount -   a)),
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Years(amount), TT::Moment(m)))),
        TT::Addition(b, c)    => sub_from_years(amount, try!(add(b, c))),
        TT::Subtraction(b, c) => sub_from_years(amount, try!(sub(b, c))),
    }
}

#[inline]
fn adjust_times_sub(mut y: i64, mut mo: i64, mut d: i64, mut h: i64, mut mi: i64, mut s: i64)
    -> (i64, i64, i64, i64, i64, i64)
{
    macro_rules! fix {
        {
            $base:ident,
            $next:ident
        } => {
            if $base < 0 {
                $next += $base;
                $base = 0;
            }
        }
    }

    fix! { s , mi }
    fix! { mi, h  }
    fix! { h , d  }
    fix! { d , mo }
    fix! { mo, y  }

    (y, mo, d, h, mi, s)
}

fn sub_from_moment(mom: NaiveDateTime, tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Seconds(a)        => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64 - a;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Minutes(a)        => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64 - a;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Hours(a)          => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64 - a;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Days(a)           => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64 - a;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Months(a)         => {
            let y  = mom.year() as i64;
            let mo = mom.month() as i64 - a;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Years(a)          => {
            let y  = mom.year() as i64 - a;
            let mo = mom.month() as i64;
            let d  = mom.day() as i64;
            let h  = mom.hour() as i64;
            let mi = mom.minute() as i64;
            let s  = mom.second() as i64;

            let (y, mo, d, h, mi, s) = adjust_times_sub(y, mo, d, h, mi, s);

            let tt = NaiveDate::from_ymd(y as i32, mo as u32, d as u32)
                  .and_hms(h as u32, mi as u32, s as u32);
            Ok(TimeType::moment(tt))
        },
        TT::Moment(m)         => Err(KE::from_kind(KEK::CannotAdd(TT::Moment(mom), TT::Moment(m)))),
        TT::Addition(a, b)    => sub_from_moment(mom, try!(add(a, b))),
        TT::Subtraction(a, b) => sub_from_moment(mom, try!(sub(a, b))),
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::TimeType as TT;

    use error::KairosErrorKind as KEK;

    #[test]
    fn test_addition_of_seconds() {
        let a = TT::seconds(0);
        let b = TT::seconds(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_seconds());
                assert_eq!(1, b.get_seconds());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_seconds_multiple() {
        let a = TT::seconds(0);
        let b = TT::seconds(1);
        let c = TT::seconds(2);

        let d = a + b + c;

        match d {
            TT::Addition(add, c) => {
                match *add {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_seconds());
                        assert_eq!(1, b.get_seconds());
                        assert_eq!(2, c.get_seconds());
                    },
                    _ => assert!(false, "Addition failed, returned non-Addition type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds() {
        let a = TT::seconds(5);
        let b = TT::seconds(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_seconds());
                assert_eq!(3, b.get_seconds());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds_multiple() {
        let a = TT::seconds(3);
        let b = TT::seconds(2);
        let c = TT::seconds(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_seconds());
                        assert_eq!(2, b.get_seconds());
                        assert_eq!(1, c.get_seconds());
                    },
                    _ => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_seconds_calculate() {
        let a = TT::seconds(0);
        let b = TT::seconds(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_seconds());
    }

    #[test]
    fn test_addition_of_seconds_multiple_calculate() {
        let a = TT::seconds(0);
        let b = TT::seconds(1);
        let c = TT::seconds(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_seconds());
    }

    #[test]
    fn test_subtraction_of_seconds_calculate() {
        let a = TT::seconds(5);
        let b = TT::seconds(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_seconds());
    }

    #[test]
    fn test_subtraction_of_seconds_multiple_calculate() {
        let a = TT::seconds(3);
        let b = TT::seconds(2);
        let c = TT::seconds(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_seconds());
    }

    #[test]
    fn test_addition_of_minutes() {
        let a = TT::minutes(0);
        let b = TT::minutes(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_minutes());
                assert_eq!(1, b.get_minutes());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_minutes_multiple() {
        let a = TT::minutes(0);
        let b = TT::minutes(1);
        let c = TT::minutes(2);

        let d = a + b + c;

        match d {
            TT::Addition(ref add, ref c) => {
                match **add {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_minutes());
                        assert_eq!(1, b.get_minutes());
                        assert_eq!(2, c.get_minutes());
                    },
                    _ => assert!(false, "Addition failed, returned non-Addition type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes() {
        let a = TT::minutes(5);
        let b = TT::minutes(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_minutes());
                assert_eq!(3, b.get_minutes());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes_multiple() {
        let a = TT::minutes(3);
        let b = TT::minutes(2);
        let c = TT::minutes(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_minutes());
                        assert_eq!(2, b.get_minutes());
                        assert_eq!(1, c.get_minutes());
                    },
                    _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_minutes_calculate() {
        let a = TT::minutes(0);
        let b = TT::minutes(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_minutes());
    }

    #[test]
    fn test_addition_of_minutes_multiple_calculate() {
        let a = TT::minutes(0);
        let b = TT::minutes(1);
        let c = TT::minutes(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_minutes());
    }

    #[test]
    fn test_subtraction_of_minutes_calculate() {
        let a = TT::minutes(5);
        let b = TT::minutes(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_minutes());
    }

    #[test]
    fn test_subtraction_of_minutes_multiple_calculate() {
        let a = TT::minutes(3);
        let b = TT::minutes(2);
        let c = TT::minutes(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_minutes());
    }

    #[test]
    fn test_addition_of_days() {
        let a = TT::days(0);
        let b = TT::days(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_days());
                assert_eq!(1, b.get_days());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_days_multiple() {
        let a = TT::days(0);
        let b = TT::days(1);
        let c = TT::days(2);

        let d = a + b + c;

        match d {
            TT::Addition(add, c) => {
                match *add {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_days());
                        assert_eq!(1, b.get_days());
                        assert_eq!(2, c.get_days());
                    },
                    _ => assert!(false, "Addition failed, wrong type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_days() {
        let a = TT::days(5);
        let b = TT::days(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_days());
                assert_eq!(3, b.get_days());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_days_multiple() {
        let a = TT::days(3);
        let b = TT::days(2);
        let c = TT::days(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_days());
                        assert_eq!(2, b.get_days());
                        assert_eq!(1, c.get_days());
                    },
                    _ => assert!(false, "Subtraction failed, wrong type"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_days_calculate() {
        let a = TT::days(0);
        let b = TT::days(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_days());
    }

    #[test]
    fn test_addition_of_days_multiple_calculate() {
        let a = TT::days(0);
        let b = TT::days(1);
        let c = TT::days(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_days());
    }

    #[test]
    fn test_subtraction_of_days_calculate() {
        let a = TT::days(5);
        let b = TT::days(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_days());
    }

    #[test]
    fn test_subtraction_of_days_multiple_calculate() {
        let a = TT::days(3);
        let b = TT::days(2);
        let c = TT::days(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_days());
    }

    #[test]
    fn test_addition_of_months() {
        let a = TT::months(0);
        let b = TT::months(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_months());
                assert_eq!(1, b.get_months());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_months_multiple() {
        let a = TT::months(0);
        let b = TT::months(1);
        let c = TT::months(2);

        let d = a + b + c;

        match d {
            TT::Addition(add, c) => {
                match *add {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_months());
                        assert_eq!(1, b.get_months());
                        assert_eq!(2, c.get_months());
                    },
                    _ => assert!(false, "Addition failed, wrong type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_months() {
        let a = TT::months(5);
        let b = TT::months(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_months());
                assert_eq!(3, b.get_months());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_months_multiple() {
        let a = TT::months(3);
        let b = TT::months(2);
        let c = TT::months(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_months());
                        assert_eq!(2, b.get_months());
                        assert_eq!(1, c.get_months());
                    },
                    _ => assert!(false, "Subtraction failed, wrong type"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_months_calculate() {
        let a = TT::months(0);
        let b = TT::months(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_months());
    }

    #[test]
    fn test_addition_of_months_multiple_calculate() {
        let a = TT::months(0);
        let b = TT::months(1);
        let c = TT::months(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_months());
    }

    #[test]
    fn test_subtraction_of_months_calculate() {
        let a = TT::months(5);
        let b = TT::months(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_months());
    }

    #[test]
    fn test_subtraction_of_months_multiple_calculate() {
        let a = TT::months(3);
        let b = TT::months(2);
        let c = TT::months(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_months());
    }

    #[test]
    fn test_addition_of_years() {
        let a = TT::years(0);
        let b = TT::years(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_years());
                assert_eq!(1, b.get_years());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_years_multiple() {
        let a = TT::years(0);
        let b = TT::years(1);
        let c = TT::years(2);

        let d = a + b + c;

        match d {
            TT::Addition(add, c) => {
                match *add {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_years());
                        assert_eq!(1, b.get_years());
                        assert_eq!(2, c.get_years());
                    },
                    _ => assert!(false, "Addition failed, wrong type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_years() {
        let a = TT::years(5);
        let b = TT::years(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_years());
                assert_eq!(3, b.get_years());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_years_multiple() {
        let a = TT::years(3);
        let b = TT::years(2);
        let c = TT::years(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_years());
                        assert_eq!(2, b.get_years());
                        assert_eq!(1, c.get_years());
                    },
                    _ => assert!(false, "Subtraction failed, wrong type"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_years_calculate() {
        let a = TT::years(0);
        let b = TT::years(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_years());
    }

    #[test]
    fn test_addition_of_years_multiple_calculate() {
        let a = TT::years(0);
        let b = TT::years(1);
        let c = TT::years(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_years());
    }

    #[test]
    fn test_subtraction_of_years_calculate() {
        let a = TT::years(5);
        let b = TT::years(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_years());
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate() {
        let a = TT::years(3);
        let b = TT::years(2);
        let c = TT::years(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_years());
    }

    #[test]
    fn test_addition_of_years_multiple_calculate_reverse_order() {
        let a = TT::years(0);
        let b = TT::years(1);
        let c = TT::years(2);

        let d = (a + (b + c)).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_years());
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate_reverse_order() {
        let a = TT::years(3);
        let b = TT::years(2);
        let c = TT::years(1);

        let d = (a - (b - c)).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(2, d.get_years());
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate_reverse_order_2() {
        let a = TT::years(3);
        let b = TT::years(2);
        let c = TT::years(1);
        let d = TT::years(10);

        let e = ((d - c) - (a - b)).calculate();

        assert!(e.is_ok());
        let e = e.unwrap();

        assert_eq!(8, e.get_years());
    }

    #[test]
    fn test_add_moment_to_seconds() {
        let a = TT::seconds(3);
        let b = TT::moment(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11));

        let res = (a + b).calculate();

        assert!(res.is_err());
        let res = res.unwrap_err();

        assert_eq!("Cannot add", res.kind().description());
    }

    #[test]
    fn test_subtract_moment_from_seconds() {
        let a = TT::seconds(3);
        let b = TT::moment(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11));

        let res = (a - b).calculate();

        assert!(res.is_err());
        let res = res.unwrap_err();

        assert_eq!("Cannot subtract", res.kind().description());
    }

}

#[cfg(test)]
mod test_add_and_sub_mixed {
    use timetype::TimeType as TT;

    #[test]
    fn test_add_then_sub() {
        let a = TT::seconds(0);
        let b = TT::seconds(1);
        let c = TT::seconds(1);

        let d = a + b - c;

        assert_eq!(0, d.calculate().unwrap().get_seconds());
    }
    #[test]
    fn test_sub_then_add() {
        let a = TT::seconds(1);
        let b = TT::seconds(1);
        let c = TT::seconds(0);

        let d = a - b + c;

        assert_eq!(0, d.calculate().unwrap().get_seconds());
    }
}

#[cfg(test)]
mod timetype_value_tests {
    use super::TimeType as TT;

    #[test]
    fn test_set_seconds_get_others() {
        let t = TT::seconds(59);

        assert_eq!(59, t.get_seconds());
        assert_eq!(0, t.get_minutes());
        assert_eq!(0, t.get_hours());
        assert_eq!(0, t.get_days());
        assert_eq!(0, t.get_months());
        assert_eq!(0, t.get_years());
    }

    #[test]
    fn test_set_minutes_get_others() {
        let t = TT::minutes(59);

        assert_eq!(59 * 60, t.get_seconds());
        assert_eq!(59, t.get_minutes());
        assert_eq!(0, t.get_hours());
        assert_eq!(0, t.get_days());
        assert_eq!(0, t.get_months());
        assert_eq!(0, t.get_years());
    }

    #[test]
    fn test_set_hours_get_others() {
        let t = TT::hours(59);

        assert_eq!(59 * 60 * 60, t.get_seconds());
        assert_eq!(59 * 60, t.get_minutes());
        assert_eq!(59, t.get_hours());
        assert_eq!(2, t.get_days());
        assert_eq!(0, t.get_months());
        assert_eq!(0, t.get_years());
    }

    #[test]
    fn test_set_days_get_others() {
        let t = TT::days(59);

        assert_eq!(59 * 24 * 60 * 60, t.get_seconds());
        assert_eq!(59 * 24 * 60, t.get_minutes());
        assert_eq!(59 * 24, t.get_hours());
        assert_eq!(59, t.get_days());
        assert_eq!(1, t.get_months());
        assert_eq!(0, t.get_years());
    }

    #[test]
    fn test_set_weeks_get_others() {
        let t = TT::weeks(59);

        assert_eq!(59 * 7 * 24 * 60 * 60, t.get_seconds());
        assert_eq!(59 * 7 * 24 * 60, t.get_minutes());
        assert_eq!(59 * 7 * 24, t.get_hours());
        assert_eq!(59 * 7, t.get_days());
        assert_eq!(13, t.get_months());
        assert_eq!(1, t.get_years());
    }

    #[test]
    fn test_set_months_get_others() {
        let t = TT::months(59);

        assert_eq!(59 * 30 * 24 * 60 * 60, t.get_seconds());
        assert_eq!(59 * 30 * 24 * 60, t.get_minutes());
        assert_eq!(59 * 30 * 24, t.get_hours());
        assert_eq!(59 * 30, t.get_days());
        assert_eq!(59, t.get_months());
        assert_eq!(4, t.get_years());
    }

    #[test]
    fn test_set_years_get_others() {
        let t = TT::years(59);

        assert_eq!(59 * 12 * 30 * 24 * 60 * 60, t.get_seconds());
        assert_eq!(59 * 12 * 30 * 24 * 60, t.get_minutes());
        assert_eq!(59 * 12 * 30 * 24, t.get_hours());
        assert_eq!(59 * 12 * 30, t.get_days());
        assert_eq!(59 * 12, t.get_months());
        assert_eq!(59, t.get_years());
    }


}

#[cfg(test)]
mod moment_plus_amount_tests {
    use super::TimeType as TT;
    use chrono::NaiveDate;
    use chrono::Timelike;
    use chrono::Datelike;

    macro_rules! generate_test_moment_operator_amount{
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
            operator = $op:expr;
        } => {
            #[test]
            fn $name() {
                let base = TT::moment($base);
                let result = $op(base, $amount).calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base - amount;
            }
        }
    }

    //
    // tests
    //

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 1);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 2);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 2, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 5, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 2);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 3, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 1, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(23, 59, 59);
    }

}

#[cfg(test)]
mod test_time_adjustments {
    use super::adjust_times_add;
    use super::adjust_times_sub;

    macro_rules! generate_test_add {
        {
            y : $y  :expr => $then_y  :expr;
            mo: $mo :expr => $then_mo :expr;
            d : $d  :expr => $then_d  :expr;
            h : $h  :expr => $then_h  :expr;
            m : $m  :expr => $then_m  :expr;
            s : $s  :expr => $then_s  :expr;
        } => {
            let (y, mo, d, h, mi, s) = adjust_times_add($y, $mo, $d, $h, $m, $s);

            assert_eq!($then_y , y );
            assert_eq!($then_mo, mo);
            assert_eq!($then_d , d );
            assert_eq!($then_h , h );
            assert_eq!($then_m , mi);
            assert_eq!($then_s , s );
        }
    }

    macro_rules! generate_test_sub {
        {
            y : $y  :expr => $then_y  :expr;
            mo: $mo :expr => $then_mo :expr;
            d : $d  :expr => $then_d  :expr;
            h : $h  :expr => $then_h  :expr;
            m : $m  :expr => $then_m  :expr;
            s : $s  :expr => $then_s  :expr;
        } => {
            let (y, mo, d, h, mi, s) = adjust_times_sub($y, $mo, $d, $h, $m, $s);

            assert_eq!($then_y , y );
            assert_eq!($then_mo, mo);
            assert_eq!($then_d , d );
            assert_eq!($then_h , h );
            assert_eq!($then_m , mi);
            assert_eq!($then_s , s );
        }
    }

    #[test]
    fn test_adjust_times_add_seconds() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  0 =>  0;
            d  :  0 =>  0;
            h  :  0 =>  0;
            m  :  0 =>  1;
            s  : 62 =>  2;
        }
    }

    #[test]
    fn test_adjust_times_add_minutes() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  0 =>  0;
            d  :  0 =>  0;
            h  :  0 =>  1;
            m  : 62 =>  2;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_add_hours() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  0 =>  0;
            d  :  0 =>  1;
            h  : 26 =>  2;
            m  :  0 =>  0;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_add_days() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  0 =>  1;
            d  : 32 =>  2;
            h  :  0 =>  0;
            m  :  0 =>  0;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_add_months() {
        generate_test_add! {
            y  :  0 =>  1;
            mo : 14 =>  2;
            d  :  0 =>  0;
            h  :  0 =>  0;
            m  :  0 =>  0;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_sub_seconds() {
        generate_test_sub! {
            y  :  1 -  0 =>  0;
            mo :  0 -  1 => 11;
            d  :  0 -  0 =>  0;
            h  :  0 -  0 =>  0;
            m  :  0 -  0 =>  0;
            s  :  0 -  0 =>  0;
        }
    }

}

