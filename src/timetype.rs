//! The module for the TimeType
//!

use chrono::NaiveDateTime;

use std::ops::Add;
use std::ops::Sub;

use result::Result;
use error::KairosErrorKind as KEK;
use error::KairosError as KE;
use error_chain::ChainedError;

/// A Type of Time, currently based on chrono::NaiveDateTime
#[derive(Debug, Clone)]
pub enum TimeType {
    Duration(::chrono::Duration),

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

impl Sub for TimeType {
    type Output = TimeType;

    fn sub(self, rhs: TimeType) -> Self::Output {
        TimeType::Subtraction(Box::new(self), Box::new(rhs))
    }
}

impl TimeType {

    pub fn seconds(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::seconds(i))
    }

    pub fn minutes(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::minutes(i))
    }

    pub fn hours(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::hours(i))
    }

    pub fn days(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::days(i))
    }

    pub fn weeks(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::weeks(i))
    }

    pub fn months(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::weeks(i * 4))
    }

    pub fn years(i: i64) -> TimeType {
        TimeType::Duration(::chrono::Duration::weeks(i * 4 * 12))
    }

    pub fn moment(ndt: NaiveDateTime) -> TimeType {
        TimeType::Moment(ndt)
    }

    /// Get the number of seconds, if the TimeType is not a seconds type, zero is returned
    pub fn get_seconds(&self) -> i64 {
        match *self {
            TimeType::Duration(d)   => d.num_seconds(),
            _                       => 0
        }
    }

    /// Get the number of minutes, if the TimeType is not a minutes type, zero is returned
    pub fn get_minutes(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_minutes(),
            _ => 0,
        }
    }

    /// Get the number of hours, if the TimeType is not a hours type, zero is returned
    pub fn get_hours(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_hours(),
            _ => 0,
        }
    }

    /// Get the number of days, if the TimeType is not a days type, zero is returned
    pub fn get_days(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_days(),
            _ => 0,
        }
    }

    /// Get the number of weeks, if the TimeType is not a weeks type, zero is returned
    pub fn get_weeks(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_weeks(),
            _ => 0,
        }
    }

    /// Get the number of months, if the TimeType is not a months type, zero is returned
    pub fn get_months(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_weeks() / 4,
            _ => 0,
        }
    }

    /// Get the number of years, if the TimeType is not a years type, zero is returned
    pub fn get_years(&self) -> i64 {
        match *self {
            TimeType::Duration(d) => d.num_weeks() / 12 / 4,
            _ => 0,
        }
    }

    fn calculate(self) -> Result<TimeType> {
        use timetype::TimeType as TT;

        match self {
            TT::Addition(a, b)    => add(a, b),
            TT::Subtraction(a, b) => sub(a, b),
            x                     => Ok(x)
        }
    }
}

fn add(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Duration(a), TT::Duration(b)) => Ok(TT::Duration(a + b)),
        (TT::Addition(a, b), other)      => add(a, b)
            .map(Box::new)
            .and_then(|bx| add(bx, Box::new(other))),
        (other, TT::Addition(a, b))      => add(a, b)
            .map(Box::new)
            .and_then(|bx| add(Box::new(other), bx)),
        (thing, TT::Moment(mom)) => Err(KE::from_kind(KEK::CannotAdd(thing, TT::Moment(mom)))),
        others                           => unimplemented!(),
    }
}

fn sub(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Duration(a), TT::Duration(b)) => Ok(TT::Duration(a - b)),
        (TT::Subtraction(a, b), other)   => sub(a, b)
            .map(Box::new)
            .and_then(|bx| sub(bx, Box::new(other))),
        (other, TT::Subtraction(a, b))   => sub(a, b)
            .map(Box::new)
            .and_then(|bx| sub(Box::new(other), bx)),
        (thing, TT::Moment(mom)) => Err(KE::from_kind(KEK::CannotSub(thing, TT::Moment(mom)))),
        others                           => unimplemented!(),
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
    fn test_addition_of_weeks() {
        let a = TT::weeks(0);
        let b = TT::weeks(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                assert_eq!(0, a.get_weeks());
                assert_eq!(1, b.get_weeks());
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_weeks_multiple() {
        let a = TT::weeks(0);
        let b = TT::weeks(1);
        let c = TT::weeks(2);

        let d = a + b + c;

        match d {
            TT::Addition(sub, c) => {
                match *sub {
                    TT::Addition(ref a, ref b) => {
                        assert_eq!(0, a.get_weeks());
                        assert_eq!(1, b.get_weeks());
                        assert_eq!(2, c.get_weeks());
                    },
                    _ => assert!(false, "Addition failed, wrong type"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks() {
        let a = TT::weeks(5);
        let b = TT::weeks(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                assert_eq!(5, a.get_weeks());
                assert_eq!(3, b.get_weeks());
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks_multiple() {
        let a = TT::weeks(3);
        let b = TT::weeks(2);
        let c = TT::weeks(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(sub, c) => {
                match *sub {
                    TT::Subtraction(ref a, ref b) => {
                        assert_eq!(3, a.get_weeks());
                        assert_eq!(2, b.get_weeks());
                        assert_eq!(1, c.get_weeks());
                    },
                    _ => assert!(false, "Subtraction failed, wrong type"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_weeks_calculate() {
        let a = TT::weeks(0);
        let b = TT::weeks(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(1, c.get_weeks());
    }

    #[test]
    fn test_addition_of_weeks_multiple_calculate() {
        let a = TT::weeks(0);
        let b = TT::weeks(1);
        let c = TT::weeks(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(3, d.get_weeks());
    }

    #[test]
    fn test_subtraction_of_weeks_calculate() {
        let a = TT::weeks(5);
        let b = TT::weeks(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        assert_eq!(2, c.get_weeks());
    }

    #[test]
    fn test_subtraction_of_weeks_multiple_calculate() {
        let a = TT::weeks(3);
        let b = TT::weeks(2);
        let c = TT::weeks(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        assert_eq!(0, d.get_weeks());
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


