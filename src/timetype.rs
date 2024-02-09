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

use error::Result;
use error::Error;
use indicator::{Day, Month};
use util::*;

/// A Type of Time, currently based on chrono::NaiveDateTime
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    EndOfYear(Box<TimeType>),
    EndOfMonth(Box<TimeType>),
    EndOfDay(Box<TimeType>),
    EndOfHour(Box<TimeType>),
    EndOfMinute(Box<TimeType>),
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

    /// Alias for `TimeType::moment(::chrono::offset::Local::now().naive_local())`
    pub fn today() -> TimeType {
        TimeType::moment(::chrono::offset::Local::now().naive_local())
    }

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

    /// Calculate the end of the year based on the current TimeType
    ///
    /// The end of a year is considered to be the last day of the year, not the last second.
    ///
    /// # Warning
    ///
    /// If the current TimeType does _not_ evaluate to a `TimeType::Moment`, calculating the end of
    /// the year will fail
    ///
    pub fn end_of_year(self) -> TimeType {
        TimeType::EndOfYear(Box::new(self))
    }

    /// Calculate the end of the month based on the current TimeType
    ///
    /// The end of a month is considered to be the last day of the month, not the last second.
    ///
    /// # Warning
    ///
    /// If the current TimeType does _not_ evaluate to a `TimeType::Moment`, calculating the end of
    /// the month will fail
    pub fn end_of_month(self) -> TimeType {
        TimeType::EndOfMonth(Box::new(self))
    }

    /// Calculate the end of the day based on the current TimeType
    ///
    /// The end of a day is considered the last second of the day
    ///
    /// # Warning
    ///
    /// If the current TimeType does _not_ evaluate to a `TimeType::Moment`, calculating the end of
    /// the day will fail
    pub fn end_of_day(self) -> TimeType {
        TimeType::EndOfDay(Box::new(self))
    }

    /// Calculate the end of the hour based on the current TimeType
    ///
    /// The end of a hour is considered the last second of a hour
    ///
    /// # Warning
    ///
    /// If the current TimeType does _not_ evaluate to a `TimeType::Moment`, calculating the end of
    /// the hour will fail
    pub fn end_of_hour(self) -> TimeType {
        TimeType::EndOfHour(Box::new(self))
    }

    /// Calculate the end of the minute based on the current TimeType
    ///
    /// The end of a minute is considered to be the last second of a minute
    ///
    /// # Warning
    ///
    /// If the current TimeType does _not_ evaluate to a `TimeType::Moment`, calculating the end of
    /// the minute will fail
    pub fn end_of_minute(self) -> TimeType {
        TimeType::EndOfMinute(Box::new(self))
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

    /// Check whether a `TimeType::Moment` is a certain weekday. Returns an error if TimeType is
    /// not a `TimeType::Moment`.
    pub fn is_a(&self, d: Day) -> Result<bool> {
        use self::TimeType as TT;

        match *self {
            TT::Moment(m) => Ok(m.weekday() == d.into()),
            _             => Err(Error::CannotCompareDayTo(self.name())),
        }
    }

    /// Check whether a `TimeType::Moment` is in a certain month. Returns an error if the TimeType
    /// is not a `TimeType::Moment`.
    pub fn is_in(&self, month: Month) -> Result<bool> {
        use self::TimeType as TT;

        match *self {
            TT::Moment(m) => Ok(m.month() == month.into()),
            _             => Err(Error::CannotCompareMonthTo(self.name())),
        }
    }

    /// Get a string representation of the variant of the `TimeType` instance.
    pub fn name(&self) -> &'static str {
        use self::TimeType as TT;

        match *self {
            TT::Addition(..)    => "Addition",
            TT::Days(..)        => "Days",
            TT::EndOfDay(..)    => "EndOfDay",
            TT::EndOfHour(..)   => "EndOfHour",
            TT::EndOfMinute(..) => "EndOfMinute",
            TT::EndOfMonth(..)  => "EndOfMonth",
            TT::EndOfYear(..)   => "EndOfYear",
            TT::Hours(..)       => "Hours",
            TT::Minutes(..)     => "Minutes",
            TT::Moment(..)      => "Moment",
            TT::Months(..)      => "Months",
            TT::Seconds(..)     => "Seconds",
            TT::Subtraction(..) => "Subtraction",
            TT::Years(..)       => "Years",
        }
    }

    pub fn calculate(self) -> Result<TimeType> {
        do_calculate(self)
    }

}

/// Helper trait for converting things into a TimeType object
///
/// Until `TryInto` is stabilized in Rust, we need a helper trait for this.
pub trait IntoTimeType {
    fn into_timetype(self) -> Result<TimeType>;
}

impl IntoTimeType for TimeType {
    fn into_timetype(self) -> Result<TimeType> {
        Ok(self)
    }
}

fn do_calculate(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match tt {
        TT::Addition(a, b)     => add(a, b),
        TT::Subtraction(a, b)  => sub(a, b),
        TT::EndOfYear(inner)   => end_of_year(*inner),
        TT::EndOfMonth(inner)  => end_of_month(*inner),
        TT::EndOfDay(inner)    => end_of_day(*inner),
        TT::EndOfHour(inner)   => end_of_hour(*inner),
        TT::EndOfMinute(inner) => end_of_minute(*inner),
        x                      => Ok(x)
    }
}

/// Evaluates the passed argument and if it is a `TT::Moment` it adjust its to the end of the year
/// else b, cit returns an error
///
/// Calling a end-of-year on a end-of-year yields end-of-year applied only once.
fn end_of_year(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match do_calculate(tt)? {
        els @ TT::Seconds(_)        |
        els @ TT::Minutes(_)        |
        els @ TT::Hours(_)          |
        els @ TT::Days(_)           |
        els @ TT::Months(_)         |
        els @ TT::Years(_)          |
        els @ TT::Addition(_, _)    |
        els @ TT::Subtraction(_, _) => Err(Error::CannotCalculateEndOfYearOn(els)),
        TT::Moment(m) => NaiveDate::from_ymd_opt(m.year(), 12, 31)
            .map(|nd| nd.and_hms(0, 0, 0))
            .map(TT::moment)
            .ok_or(Error::OutOfBounds(m.year() as i32, 12, 31, 0, 0, 0))
            .map_err(Error::from),

        TT::EndOfYear(e)   => do_calculate(*e),
        TT::EndOfMonth(e)  => do_calculate(*e),
        TT::EndOfDay(e)    => do_calculate(*e),
        TT::EndOfHour(e)   => do_calculate(*e),
        TT::EndOfMinute(e) => do_calculate(*e),
    }
}

/// Evaluates the passed argument and if it is a `TT::Moment` it adjust its to the end of the month
/// else returns an error
///
/// Calling a end-of-month on a end-of-month yields end-of-month applied only once.
fn end_of_month(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match do_calculate(tt)? {
        els @ TT::Seconds(_)        |
        els @ TT::Minutes(_)        |
        els @ TT::Hours(_)          |
        els @ TT::Days(_)           |
        els @ TT::Months(_)         |
        els @ TT::Years(_)          |
        els @ TT::Addition(_, _)    |
        els @ TT::Subtraction(_, _) => Err(Error::CannotCalculateEndOfMonthOn(els)),
        TT::Moment(m)    => {
            let last_day = get_num_of_days_in_month(m.year() as i64, m.month() as i64) as u32;
            NaiveDate::from_ymd_opt(m.year(), m.month(), last_day)
                .map(|nd| nd.and_hms(0, 0, 0))
                .map(TT::moment)
                .ok_or(Error::OutOfBounds(m.year() as i32, m.month() as u32, last_day, 0, 0, 0))
                .map_err(Error::from)
        },
        TT::EndOfYear(e)   => do_calculate(*e),
        TT::EndOfMonth(e)  => do_calculate(*e),
        TT::EndOfDay(e)    => do_calculate(*e),
        TT::EndOfHour(e)   => do_calculate(*e),
        TT::EndOfMinute(e) => do_calculate(*e),
    }
}

/// Evaluates the passed argument and if it is a `TT::Moment` it adjust its to the end of the day
/// else returns an error
///
/// Calling a end-of-day on a end-of-day yields end-of-month applied only once.
fn end_of_day(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match do_calculate(tt)? {
        els @ TT::Seconds(_)        |
        els @ TT::Minutes(_)        |
        els @ TT::Hours(_)          |
        els @ TT::Days(_)           |
        els @ TT::Months(_)         |
        els @ TT::Years(_)          |
        els @ TT::Addition(_, _)    |
        els @ TT::Subtraction(_, _) => Err(Error::CannotCalculateEndOfMonthOn(els)),
        TT::Moment(m) => NaiveDate::from_ymd_opt(m.year(), m.month(), m.day())
            .map(|nd| nd.and_hms(23, 59, 59))
            .map(TT::moment)
            .ok_or(Error::OutOfBounds(m.year() as i32, m.month() as u32, m.day() as u32, 23, 59, 59))
            .map_err(Error::from),
        TT::EndOfYear(e)   => do_calculate(*e),
        TT::EndOfMonth(e)  => do_calculate(*e),
        TT::EndOfDay(e)    => do_calculate(*e),
        TT::EndOfHour(e)   => do_calculate(*e),
        TT::EndOfMinute(e) => do_calculate(*e),
    }
}

/// Evaluates the passed argument and if it is a `TT::Moment` it adjust its to the end of the hour
/// else returns an error
///
/// Calling a end-of-hour on a end-of-hour yields end-of-month applied only once.
fn end_of_hour(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match do_calculate(tt)? {
        els @ TT::Seconds(_)        |
        els @ TT::Minutes(_)        |
        els @ TT::Hours(_)          |
        els @ TT::Days(_)           |
        els @ TT::Months(_)         |
        els @ TT::Years(_)          |
        els @ TT::Addition(_, _)    |
        els @ TT::Subtraction(_, _) => Err(Error::CannotCalculateEndOfMonthOn(els)),
        TT::Moment(m)    => NaiveDate::from_ymd_opt(m.year(), m.month(), m.day())
            .and_then(|nd| nd.and_hms_opt(m.hour(), 59, 59))
            .map(TT::moment)
            .ok_or(Error::OutOfBounds(m.year() as i32, m.month() as u32, m.day() as u32, m.hour() as u32, 59, 59))
            .map_err(Error::from),
        TT::EndOfYear(e)   => do_calculate(*e),
        TT::EndOfMonth(e)  => do_calculate(*e),
        TT::EndOfDay(e)    => do_calculate(*e),
        TT::EndOfHour(e)   => do_calculate(*e),
        TT::EndOfMinute(e) => do_calculate(*e),
    }
}

/// Evaluates the passed argument and if it is a `TT::Moment` it adjust its to the end of the
/// minute else returns an error
///
/// Calling a end-of-minute on a end-of-minute yields end-of-month applied only once.
fn end_of_minute(tt: TimeType) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match do_calculate(tt)? {
        els @ TT::Seconds(_)        |
        els @ TT::Minutes(_)        |
        els @ TT::Hours(_)          |
        els @ TT::Days(_)           |
        els @ TT::Months(_)         |
        els @ TT::Years(_)          |
        els @ TT::Addition(_, _)    |
        els @ TT::Subtraction(_, _) => Err(Error::CannotCalculateEndOfMonthOn(els)),
        TT::Moment(m)    => NaiveDate::from_ymd_opt(m.year(), m.month(), m.day())
            .and_then(|nd| nd.and_hms_opt(m.hour(), m.minute(), 59))
            .map(TT::moment)
            .ok_or(Error::OutOfBounds(m.year() as i32, m.month() as u32, m.day() as u32, m.hour() as u32, m.minute() as u32, 59 as u32))
            .map_err(Error::from),
        TT::EndOfYear(e)   => do_calculate(*e),
        TT::EndOfMonth(e)  => do_calculate(*e),
        TT::EndOfDay(e)    => do_calculate(*e),
        TT::EndOfHour(e)   => do_calculate(*e),
        TT::EndOfMinute(e) => do_calculate(*e),
    }
}

fn add(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Moment(mom), thing) => add_to_moment(mom, thing),
        (thing, TT::Moment(mom)) => Err(Error::CannotAdd(thing, TT::Moment(mom))),

        (TT::Seconds(a), other) => add_to_seconds(a, other),
        (TT::Minutes(a), other) => add_to_minutes(a, other),
        (TT::Hours(a), other)   => add_to_hours(a, other),
        (TT::Days(a), other)    => add_to_days(a, other),
        (TT::Months(a), other)  => add_to_months(a, other),
        (TT::Years(a), other)   => add_to_years(a, other),

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

        (TT::EndOfYear(e), other) => Err(Error::CannotAdd(other, TT::EndOfYear(e))),
        (other, TT::EndOfYear(e)) => Err(Error::CannotAdd(other, TT::EndOfYear(e))),

        (TT::EndOfMonth(e), other) => Err(Error::CannotAdd(other, TT::EndOfMonth(e))),
        (other, TT::EndOfMonth(e)) => Err(Error::CannotAdd(other, TT::EndOfMonth(e))),

        (TT::EndOfDay(e), other) => Err(Error::CannotAdd(other, TT::EndOfDay(e))),
        (other, TT::EndOfDay(e)) => Err(Error::CannotAdd(other, TT::EndOfDay(e))),

        (TT::EndOfHour(e), other) => Err(Error::CannotAdd(other, TT::EndOfHour(e))),
        (other, TT::EndOfHour(e)) => Err(Error::CannotAdd(other, TT::EndOfHour(e))),

        (TT::EndOfMinute(e), other) => Err(Error::CannotAdd(other, TT::EndOfMinute(e))),
        // unreachable, just for completeness:
        //(other, TT::EndOfMinute(e)) => Err(Error::CannotAdd(other, TT::EndOfMinute(e))),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Seconds(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Seconds(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Seconds(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Seconds(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Seconds(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Seconds(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_seconds(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_seconds(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Minutes(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Minutes(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Minutes(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Minutes(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Minutes(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Minutes(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_minutes(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_minutes(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Hours(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Hours(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Hours(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Hours(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Hours(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Hours(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_hours(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_hours(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Days(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Days(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Days(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Days(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Days(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Days(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_days(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_days(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Months(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Months(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Months(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Months(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Months(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Months(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_months(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_months(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Years(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Years(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Years(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Years(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Years(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Years(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => add_to_years(amount, add(b, c)?),
        TT::Subtraction(b, c) => add_to_years(amount, sub(b, c)?),
    }
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
            Ok(TimeType::moment(tt))
        },
        TT::Moment(m)         => Err(Error::CannotAdd(TT::Moment(mom), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotAdd(TT::Moment(mom), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotAdd(TT::Moment(mom), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotAdd(TT::Moment(mom), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotAdd(TT::Moment(mom), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotAdd(TT::Moment(mom), TT::EndOfMinute(e))),
        TT::Addition(a, b)    => add_to_moment(mom, add(a, b)?),
        TT::Subtraction(a, b) => add_to_moment(mom, sub(a, b)?),
    }
}

fn sub(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Moment(mom), thing) => sub_from_moment(mom, thing),
        (TT::Seconds(a), other)  => sub_from_seconds(a, other),
        (TT::Minutes(a), other)  => sub_from_minutes(a, other),
        (TT::Hours(a), other)    => sub_from_hours(a, other),
        (TT::Days(a), other)     => sub_from_days(a, other),
        (TT::Months(a), other)   => sub_from_months(a, other),
        (TT::Years(a), other)    => sub_from_years(a, other),

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

        (TT::EndOfYear(e), other) => Err(Error::CannotSub(other, TT::EndOfYear(e))),
        (other, TT::EndOfYear(e)) => Err(Error::CannotSub(other, TT::EndOfYear(e))),

        (TT::EndOfMonth(e), other) => Err(Error::CannotSub(other, TT::EndOfMonth(e))),
        (other, TT::EndOfMonth(e)) => Err(Error::CannotSub(other, TT::EndOfMonth(e))),

        (TT::EndOfDay(e), other) => Err(Error::CannotSub(other, TT::EndOfDay(e))),
        (other, TT::EndOfDay(e)) => Err(Error::CannotSub(other, TT::EndOfDay(e))),

        (TT::EndOfHour(e), other) => Err(Error::CannotSub(other, TT::EndOfHour(e))),
        (other, TT::EndOfHour(e)) => Err(Error::CannotSub(other, TT::EndOfHour(e))),

        (TT::EndOfMinute(e), other) => Err(Error::CannotSub(other, TT::EndOfMinute(e))),
        // unreachable, but for completeness
        //(other, TT::EndOfMinute(e)) => Err(Error::CannotSub(other, TT::EndOfMinute(e))),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Seconds(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Seconds(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Seconds(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Seconds(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Seconds(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Seconds(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_seconds(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_seconds(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Minutes(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Minutes(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Minutes(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Minutes(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Minutes(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Minutes(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_minutes(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_minutes(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Hours(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Hours(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Hours(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Hours(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Hours(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Hours(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_hours(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_hours(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Days(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Days(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Days(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Days(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Days(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Days(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_days(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_days(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Months(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Months(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Months(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Months(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Months(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Months(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_months(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_months(amount, sub(b, c)?),
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
        TT::Moment(m)         => Err(Error::CannotSub(TT::Years(amount), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Years(amount), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Years(amount), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Years(amount), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Years(amount), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Years(amount), TT::EndOfMinute(e))),
        TT::Addition(b, c)    => sub_from_years(amount, add(b, c)?),
        TT::Subtraction(b, c) => sub_from_years(amount, sub(b, c)?),
    }
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
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

            let tt = NaiveDate::from_ymd_opt(y as i32, mo as u32, d as u32)
                .and_then(|nd| nd.and_hms_opt(h as u32, mi as u32, s as u32))
                .ok_or(Error::OutOfBounds(y as i32, mo as u32, h as u32, h as u32, mi as u32, s as u32))
                .map_err(Error::from)?;
            Ok(TimeType::moment(tt))
        },
        TT::Moment(m)         => Err(Error::CannotSub(TT::Moment(mom), TT::Moment(m))),
        TT::EndOfYear(e)      => Err(Error::CannotSub(TT::Moment(mom), TT::EndOfYear(e))),
        TT::EndOfMonth(e)     => Err(Error::CannotSub(TT::Moment(mom), TT::EndOfMonth(e))),
        TT::EndOfDay(e)       => Err(Error::CannotSub(TT::Moment(mom), TT::EndOfDay(e))),
        TT::EndOfHour(e)      => Err(Error::CannotSub(TT::Moment(mom), TT::EndOfHour(e))),
        TT::EndOfMinute(e)    => Err(Error::CannotSub(TT::Moment(mom), TT::EndOfMinute(e))),
        TT::Addition(a, b)    => sub_from_moment(mom, add(a, b)?),
        TT::Subtraction(a, b) => sub_from_moment(mom, sub(a, b)?),
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use super::TimeType as TT;
    use error::Error;

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

        assert!(match res {
            Error::CannotAdd(..) => true,
            _ => false,
        });
    }

    #[test]
    fn test_subtract_moment_from_seconds() {
        let a = TT::seconds(3);
        let b = TT::moment(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11));

        let res = (a - b).calculate();

        assert!(res.is_err());
        let res = res.unwrap_err();

        assert!(match res {
            Error::CannotSub(..) => true,
            _ => false,
        });
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
    use env_logger;
    use super::TimeType as TT;
    use chrono::NaiveDate;

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
                debug!("Using base = {:?}", base);
                debug!("           + {:?}", $amount);
                let result = $op(base, $amount).calculate();
                debug!("        -> = {:?}", result);
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

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 1);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 1);
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

    generate_test_moment_minus_amount! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 1, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_minute_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(130);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 2, 10);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_hour_in_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(130);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(2, 10, 00);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_day_in_hours_1;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(50);
        expected = NaiveDate::from_ymd(2000, 1, 3).and_hms(2, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_day_in_hours_2;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(170);
        expected = NaiveDate::from_ymd(2000, 1, 8).and_hms(2, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_month_in_days_1;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(80);
        expected = NaiveDate::from_ymd(2000, 3,21).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_month_in_days_2;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(120);
        expected = NaiveDate::from_ymd(2000, 4,30).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_month_in_days_3;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(150);
        expected = NaiveDate::from_ymd(2000, 5,30).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_1;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(15);
        expected = NaiveDate::from_ymd(2001, 4, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_2;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(25);
        expected = NaiveDate::from_ymd(2002, 2, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_3;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(78);
        expected = NaiveDate::from_ymd(2006, 7, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_4;
        base     = NaiveDate::from_ymd(2000,10,31).and_hms(0, 0, 0);
        amount   = TT::months(4);
        expected = NaiveDate::from_ymd(2001, 3, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_5;
        base     = NaiveDate::from_ymd(2000,10,31).and_hms(0, 0, 0);
        amount   = TT::months(5);
        expected = NaiveDate::from_ymd(2001, 4, 1).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount! {
        name     = test_moment_plus_more_than_one_year_in_months_6;
        base     = NaiveDate::from_ymd(2000,10,31).and_hms(0, 0, 0);
        amount   = TT::months(4) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 4, 1).and_hms(0, 0, 0);
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

            assert_eq!($then_y , y , "Failed: y  should be {} but is {}", $then_y , y );
            assert_eq!($then_mo, mo, "Failed: mo should be {} but is {}", $then_mo, mo);
            assert_eq!($then_d , d , "Failed: d  should be {} but is {}", $then_d , d );
            assert_eq!($then_h , h , "Failed: h  should be {} but is {}", $then_h , h );
            assert_eq!($then_m , mi, "Failed: m  should be {} but is {}", $then_m , mi);
            assert_eq!($then_s , s , "Failed: s  should be {} but is {}", $then_s , s );
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

            assert_eq!($then_y , y , "Failed: y  should be {} but is {}", $then_y , y );
            assert_eq!($then_mo, mo, "Failed: mo should be {} but is {}", $then_mo, mo);
            assert_eq!($then_d , d , "Failed: d  should be {} but is {}", $then_d , d );
            assert_eq!($then_h , h , "Failed: h  should be {} but is {}", $then_h , h );
            assert_eq!($then_m , mi, "Failed: m  should be {} but is {}", $then_m , mi);
            assert_eq!($then_s , s , "Failed: s  should be {} but is {}", $then_s , s );
        }
    }

    #[test]
    fn test_adjust_times_add_seconds() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  1 =>  1;
            d  :  1 =>  1;
            h  :  0 =>  0;
            m  :  0 =>  1;
            s  : 62 =>  2;
        }
    }

    #[test]
    fn test_adjust_times_add_minutes() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  1 =>  1;
            d  :  1 =>  1;
            h  :  0 =>  1;
            m  : 62 =>  2;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_add_hours() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  1 =>  1;
            d  :  1 =>  2;
            h  : 26 =>  2;
            m  :  0 =>  0;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_add_days() {
        generate_test_add! {
            y  :  0 =>  0;
            mo :  1 =>  2;
            d  : 32 =>  1;
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
            d  :  1 =>  1;
            h  :  0 =>  0;
            m  :  0 =>  0;
            s  :  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_sub_seconds() {
        generate_test_sub! {
            y  :  1 -  0 =>  0;
            mo :  1 -  1 => 12;
            d  :  1 -  0 =>  1;
            h  :  0 -  0 =>  0;
            m  :  0 -  0 =>  0;
            s  :  0 -  0 =>  0;
        }
    }

    #[test]
    fn test_adjust_times_month_border() {
        generate_test_add! {
            y  : 2000 +  0 => 2000;
            mo :    1 +  0 =>    2;
            d  :   22 + 14 =>    5;
            h  :    0 +  0 =>    0;
            m  :    0 +  0 =>    0;
            s  :    0 +  0 =>    0;
        }

        generate_test_add! {
            y  : 2000 +  0 => 2000;
            mo :    1 +  0 =>    2;
            d  :   22 + 28 =>   19;
            h  :    0 +  0 =>    0;
            m  :    0 +  0 =>    0;
            s  :    0 +  0 =>    0;
        }

        generate_test_add! {
            y  : 2000 +  0 => 2000;
            mo :    2 +  0 =>    3;
            d  :   22 + 14 =>    7;
            h  :    0 +  0 =>    0;
            m  :    0 +  0 =>    0;
            s  :    0 +  0 =>    0;
        }

        generate_test_add! {
            y  : 2000 +  0 => 2000;
            mo :    2 +  0 =>    3;
            d  :   22 + 28 =>   21;
            h  :    0 +  0 =>    0;
            m  :    0 +  0 =>    0;
            s  :    0 +  0 =>    0;
        }

        generate_test_add! {
            y  : 2000 +  0 => 2000;
            mo :    3 +  0 =>    4;
            d  :   22 + 14 =>    5;
            h  :    0 +  0 =>    0;
            m  :    0 +  0 =>    0;
            s  :    0 +  0 =>    0;
        }
    }

}

#[cfg(test)]
mod test_end_of_year {
    use super::TimeType as TT;
    use chrono::NaiveDate;

    macro_rules! generate_test_moment_operator_amount_and_end_of_year {
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
                let result = $op(base, $amount).end_of_year().calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount_and_end_of_year {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_year! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount_and_end_of_year {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_year! {
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

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_year! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount_and_end_of_year! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 12, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount_and_end_of_year! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(00, 00, 00);
    }

    generate_test_moment_minus_amount_and_end_of_year! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(0, 0, 0);
    }

}

#[cfg(test)]
mod test_end_of_month {
    use super::TimeType as TT;
    use chrono::NaiveDate;

    macro_rules! generate_test_moment_operator_amount_and_end_of_month {
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
                let result = $op(base, $amount).end_of_month().calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount_and_end_of_month {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_month! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount_and_end_of_month {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_month! {
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

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 3, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 2, 28).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 2, 29).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 2, 28).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 2, 29).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_plus_amount_and_end_of_month! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount_and_end_of_month! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 31).and_hms(0, 0, 0);
    }

    generate_test_moment_minus_amount_and_end_of_month! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(00, 00, 00);
    }

    generate_test_moment_minus_amount_and_end_of_month! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 1, 31).and_hms(0, 0, 0);
    }

}


#[cfg(test)]
mod test_end_of_day {
    use super::TimeType as TT;
    use chrono::NaiveDate;

    macro_rules! generate_test_moment_operator_amount_and_end_of_day {
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
                let result = $op(base, $amount).end_of_day().calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount_and_end_of_day {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_day! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount_and_end_of_day {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_day! {
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

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 3, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_day! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_day! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(23, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_day! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(23, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_day! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 1, 1).and_hms(23, 59, 59);
    }

}

#[cfg(test)]
mod test_end_of_hour {
    use super::TimeType as TT;
    use chrono::NaiveDate;

    macro_rules! generate_test_moment_operator_amount_and_end_of_hour {
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
                let result = $op(base, $amount).end_of_hour().calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount_and_end_of_hour {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_hour! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount_and_end_of_hour {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_hour! {
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

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 3, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 59, 59);
    }

    generate_test_moment_plus_amount_and_end_of_hour! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_hour! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_hour! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(23, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_hour! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 1, 1).and_hms(0, 59, 59);
    }

}

#[cfg(test)]
mod test_end_of_minute {
    use super::TimeType as TT;
    use chrono::NaiveDate;

    macro_rules! generate_test_moment_operator_amount_and_end_of_minute {
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
                let result = $op(base, $amount).end_of_minute().calculate();
                assert!(result.is_ok(), "Operation failed: {:?}", result);
                let result = result.unwrap();
                let expected = $exp;

                assert_eq!(expected, *result.get_moment().unwrap());
            }
        }
    }

    macro_rules! generate_test_moment_plus_amount_and_end_of_minute {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_minute! {
                name     = $name;
                base     = $base;
                amount   = $amount;
                expected = $exp;
                operator = |base, amount| base + amount;
            }
        }
    }

    macro_rules! generate_test_moment_minus_amount_and_end_of_minute {
        {
            name     = $name:ident;
            base     = $base:expr;
            amount   = $amount:expr;
            expected = $exp:expr;
        } => {
            generate_test_moment_operator_amount_and_end_of_minute! {
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

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_zero_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_too_much_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(2);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 2, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_too_much_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(65);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 5, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_minutes_in_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(62);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(14);
        expected = NaiveDate::from_ymd(2001, 3, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_years;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(62);
        expected = NaiveDate::from_ymd(2062, 1, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_more_than_one_year;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::years(1) + TT::months(1);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_more_than_one_month;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);

        // As we calculate 1 month + 1 day first, we end up adding 31 days to the base
        amount   = TT::months(1) + TT::days(1);

        // and therefor this results in the date 2000-02-01
        // This is not that inuitive, of course.
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_more_than_one_day;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(1) + TT::hours(1);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_more_than_one_hour;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(1) + TT::minutes(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_more_than_one_minute;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(1) + TT::seconds(1);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_invalid_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(13);
        expected = NaiveDate::from_ymd(2001, 2, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_invalid_days;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::days(31);
        expected = NaiveDate::from_ymd(2000, 2, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_invalid_hours;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::hours(25);
        expected = NaiveDate::from_ymd(2000, 1, 2).and_hms(1, 0, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_invalid_minutes;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::minutes(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(1, 1, 59);
    }

    generate_test_moment_plus_amount_and_end_of_minute! {
        name     = test_moment_plus_invalid_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(61);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 1, 59);
    }

    generate_test_moment_minus_amount_and_end_of_minute! {
        name     = test_moment_minus_nothing;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(0);
        expected = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 59);
    }

    generate_test_moment_minus_amount_and_end_of_minute! {
        name     = test_moment_minus_seconds;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::seconds(1);
        expected = NaiveDate::from_ymd(1999, 12, 31).and_hms(23, 59, 59);
    }

    generate_test_moment_minus_amount_and_end_of_minute! {
        name     = test_moment_minus_months;
        base     = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        amount   = TT::months(12);
        expected = NaiveDate::from_ymd(1999, 1, 1).and_hms(0, 0, 59);
    }

}

#[cfg(test)]
mod test_is_a {
    use super::TimeType as TT;
    use chrono::NaiveDate as ND;
    use indicator::Day;

    fn ymd(y: i32, m: u32, d: u32) -> TT {
        TT::moment(ND::from_ymd(y, m, d).and_hms(0, 0, 0))
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_1() {
        assert!(ymd(2000, 1, 1).is_a(Day::Monday).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_2() {
        assert!(ymd(2000, 1, 1).is_a(Day::Tuesday).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_3() {
        assert!(ymd(2000, 1, 1).is_a(Day::Wednesday).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_4() {
        assert!(ymd(2000, 1, 1).is_a(Day::Thursday).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_5() {
        assert!(ymd(2000, 1, 1).is_a(Day::Friday).unwrap());
    }

    #[test]
    fn test_is_a_6() {
        assert!(ymd(2000, 1, 1).is_a(Day::Saturday).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_a_7() {
        assert!(ymd(2000, 1, 1).is_a(Day::Sunday).unwrap());
    }

}

#[cfg(test)]
mod test_is_in {
    use super::TimeType as TT;
    use chrono::NaiveDate as ND;
    use indicator::Month;

    fn ymd(y: i32, m: u32, d: u32) -> TT {
        TT::moment(ND::from_ymd(y, m, d).and_hms(0, 0, 0))
    }

    #[test]
    fn test_is_in_1() {
        assert!(ymd(2000, 1, 1).is_in(Month::January).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_2() {
        assert!(ymd(2000, 1, 1).is_in(Month::February).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_3() {
        assert!(ymd(2000, 1, 1).is_in(Month::March).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_4() {
        assert!(ymd(2000, 1, 1).is_in(Month::April).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_5() {
        assert!(ymd(2000, 1, 1).is_in(Month::May).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_6() {
        assert!(ymd(2000, 1, 1).is_in(Month::June).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_7() {
        assert!(ymd(2000, 1, 1).is_in(Month::July).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_8() {
        assert!(ymd(2000, 1, 1).is_in(Month::August).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_9() {
        assert!(ymd(2000, 1, 1).is_in(Month::September).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_10() {
        assert!(ymd(2000, 1, 1).is_in(Month::October).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_11() {
        assert!(ymd(2000, 1, 1).is_in(Month::November).unwrap());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_is_in_12() {
        assert!(ymd(2000, 1, 1).is_in(Month::December).unwrap());
    }

}

