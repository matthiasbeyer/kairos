//! The module for the TimeType
//!

use chrono::NaiveDateTime;

use std::ops::Add;
use std::ops::Sub;

use result::Result;

/// A Type of Time, currently based on chrono::NaiveDateTime
#[derive(Debug)]
pub enum TimeType {
    Seconds(usize),
    Minutes(usize),
    Hours(usize),
    Days(usize),
    Weeks(usize),
    Months(usize),
    Years(usize),

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
        (TT::Seconds(a), TT::Seconds(b)) => Ok(TT::Seconds(a + b)),
        (TT::Minutes(a), TT::Minutes(b)) => Ok(TT::Minutes(a + b)),
        (TT::Hours(a), TT::Hours(b))     => Ok(TT::Hours(a + b)),
        (TT::Days(a), TT::Days(b))       => Ok(TT::Days(a + b)),
        (TT::Weeks(a), TT::Weeks(b))     => Ok(TT::Weeks(a + b)),
        (TT::Months(a), TT::Months(b))   => Ok(TT::Months(a + b)),
        (TT::Years(a), TT::Years(b))     => Ok(TT::Years(a + b)),
        (TT::Addition(a, b), other)      => add(a, b)
            .map(Box::new)
            .and_then(|bx| add(bx, Box::new(other))),
        others                           => unimplemented!(),
    }
}

fn sub(a: Box<TimeType>, b: Box<TimeType>) -> Result<TimeType> {
    use timetype::TimeType as TT;

    match (*a, *b) {
        (TT::Seconds(a), TT::Seconds(b)) => Ok(TT::Seconds(a - b)),
        (TT::Minutes(a), TT::Minutes(b)) => Ok(TT::Minutes(a - b)),
        (TT::Hours(a), TT::Hours(b))     => Ok(TT::Hours(a - b)),
        (TT::Days(a), TT::Days(b))       => Ok(TT::Days(a - b)),
        (TT::Weeks(a), TT::Weeks(b))     => Ok(TT::Weeks(a - b)),
        (TT::Months(a), TT::Months(b))   => Ok(TT::Months(a - b)),
        (TT::Years(a), TT::Years(b))     => Ok(TT::Years(a - b)),
        (TT::Subtraction(a, b), other)   => sub(a, b)
            .map(Box::new)
            .and_then(|bx| sub(bx, Box::new(other))),
        others                           => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::TimeType as TT;

    #[test]
    fn test_addition_of_seconds() {
        let a = TT::Seconds(0);
        let b = TT::Seconds(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Seconds(0), TT::Seconds(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_seconds_multiple() {
        let a = TT::Seconds(0);
        let b = TT::Seconds(1);
        let c = TT::Seconds(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Seconds(2)) => match (*c, *d) {
                        (TT::Seconds(0), TT::Seconds(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds() {
        let a = TT::Seconds(5);
        let b = TT::Seconds(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Seconds(5), TT::Seconds(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds_multiple() {
        let a = TT::Seconds(3);
        let b = TT::Seconds(2);
        let c = TT::Seconds(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Seconds(1)) => match (*c, *d) {
                        (TT::Seconds(3), TT::Seconds(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_seconds_calculate() {
        let a = TT::Seconds(0);
        let b = TT::Seconds(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Seconds(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_seconds_multiple_calculate() {
        let a = TT::Seconds(0);
        let b = TT::Seconds(1);
        let c = TT::Seconds(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Seconds(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds_calculate() {
        let a = TT::Seconds(5);
        let b = TT::Seconds(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Seconds(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_seconds_multiple_calculate() {
        let a = TT::Seconds(3);
        let b = TT::Seconds(2);
        let c = TT::Seconds(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Seconds(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_minutes() {
        let a = TT::Minutes(0);
        let b = TT::Minutes(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Minutes(0), TT::Minutes(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_minutes_multiple() {
        let a = TT::Minutes(0);
        let b = TT::Minutes(1);
        let c = TT::Minutes(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Minutes(2)) => match (*c, *d) {
                        (TT::Minutes(0), TT::Minutes(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes() {
        let a = TT::Minutes(5);
        let b = TT::Minutes(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Minutes(5), TT::Minutes(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes_multiple() {
        let a = TT::Minutes(3);
        let b = TT::Minutes(2);
        let c = TT::Minutes(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Minutes(1)) => match (*c, *d) {
                        (TT::Minutes(3), TT::Minutes(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_minutes_calculate() {
        let a = TT::Minutes(0);
        let b = TT::Minutes(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Minutes(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_minutes_multiple_calculate() {
        let a = TT::Minutes(0);
        let b = TT::Minutes(1);
        let c = TT::Minutes(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Minutes(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes_calculate() {
        let a = TT::Minutes(5);
        let b = TT::Minutes(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Minutes(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_minutes_multiple_calculate() {
        let a = TT::Minutes(3);
        let b = TT::Minutes(2);
        let c = TT::Minutes(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Minutes(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_days() {
        let a = TT::Days(0);
        let b = TT::Days(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Days(0), TT::Days(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_days_multiple() {
        let a = TT::Days(0);
        let b = TT::Days(1);
        let c = TT::Days(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Days(2)) => match (*c, *d) {
                        (TT::Days(0), TT::Days(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_days() {
        let a = TT::Days(5);
        let b = TT::Days(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Days(5), TT::Days(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_days_multiple() {
        let a = TT::Days(3);
        let b = TT::Days(2);
        let c = TT::Days(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Days(1)) => match (*c, *d) {
                        (TT::Days(3), TT::Days(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_days_calculate() {
        let a = TT::Days(0);
        let b = TT::Days(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Days(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_days_multiple_calculate() {
        let a = TT::Days(0);
        let b = TT::Days(1);
        let c = TT::Days(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Days(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_days_calculate() {
        let a = TT::Days(5);
        let b = TT::Days(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Days(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_days_multiple_calculate() {
        let a = TT::Days(3);
        let b = TT::Days(2);
        let c = TT::Days(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Days(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_weeks() {
        let a = TT::Weeks(0);
        let b = TT::Weeks(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Weeks(0), TT::Weeks(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_weeks_multiple() {
        let a = TT::Weeks(0);
        let b = TT::Weeks(1);
        let c = TT::Weeks(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Weeks(2)) => match (*c, *d) {
                        (TT::Weeks(0), TT::Weeks(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks() {
        let a = TT::Weeks(5);
        let b = TT::Weeks(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Weeks(5), TT::Weeks(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks_multiple() {
        let a = TT::Weeks(3);
        let b = TT::Weeks(2);
        let c = TT::Weeks(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Weeks(1)) => match (*c, *d) {
                        (TT::Weeks(3), TT::Weeks(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_weeks_calculate() {
        let a = TT::Weeks(0);
        let b = TT::Weeks(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Weeks(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_weeks_multiple_calculate() {
        let a = TT::Weeks(0);
        let b = TT::Weeks(1);
        let c = TT::Weeks(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Weeks(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks_calculate() {
        let a = TT::Weeks(5);
        let b = TT::Weeks(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Weeks(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_weeks_multiple_calculate() {
        let a = TT::Weeks(3);
        let b = TT::Weeks(2);
        let c = TT::Weeks(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Weeks(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_months() {
        let a = TT::Months(0);
        let b = TT::Months(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Months(0), TT::Months(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_months_multiple() {
        let a = TT::Months(0);
        let b = TT::Months(1);
        let c = TT::Months(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Months(2)) => match (*c, *d) {
                        (TT::Months(0), TT::Months(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_months() {
        let a = TT::Months(5);
        let b = TT::Months(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Months(5), TT::Months(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_months_multiple() {
        let a = TT::Months(3);
        let b = TT::Months(2);
        let c = TT::Months(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Months(1)) => match (*c, *d) {
                        (TT::Months(3), TT::Months(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_months_calculate() {
        let a = TT::Months(0);
        let b = TT::Months(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Months(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_months_multiple_calculate() {
        let a = TT::Months(0);
        let b = TT::Months(1);
        let c = TT::Months(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Months(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_months_calculate() {
        let a = TT::Months(5);
        let b = TT::Months(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Months(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_months_multiple_calculate() {
        let a = TT::Months(3);
        let b = TT::Months(2);
        let c = TT::Months(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Months(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_years() {
        let a = TT::Years(0);
        let b = TT::Years(1);

        let c = a + b;

        match c {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Years(0), TT::Years(1)) => assert!(true),
                    _                                => assert!(false, "Addition failed"),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_addition_of_years_multiple() {
        let a = TT::Years(0);
        let b = TT::Years(1);
        let c = TT::Years(2);

        let d = a + b + c;

        match d {
            TT::Addition(a, b) => {
                match (*a, *b) {
                    (TT::Addition(c, d), TT::Years(2)) => match (*c, *d) {
                        (TT::Years(0), TT::Years(1)) => assert!(true),
                        _                                => assert!(false, "Addition failed"),
                    },
                    (a, b) => assert!(false, "Addition failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Addition failed, returned non-Addition type"),
        }
    }

    #[test]
    fn test_subtraction_of_years() {
        let a = TT::Years(5);
        let b = TT::Years(3);

        let c = a - b;

        match c {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Years(5), TT::Years(3)) => assert!(true),
                    _                                => assert!(false, "Subtraction failed"),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_subtraction_of_years_multiple() {
        let a = TT::Years(3);
        let b = TT::Years(2);
        let c = TT::Years(1);

        let d = a - b - c;

        match d {
            TT::Subtraction(a, b) => {
                match (*a, *b) {
                    (TT::Subtraction(c, d), TT::Years(1)) => match (*c, *d) {
                        (TT::Years(3), TT::Years(2)) => assert!(true),
                        _                                => assert!(false, "Subtraction failed"),
                    },
                    (a, b) => assert!(false, "Subtraction failed: \n a = {:?}\n b = {:?}", a, b),
                }
            }
            _ => assert!(false, "Subtraction failed, returned non-Subtraction type"),
        }
    }

    #[test]
    fn test_addition_of_years_calculate() {
        let a = TT::Years(0);
        let b = TT::Years(1);

        let c = (a + b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Years(1) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_addition_of_years_multiple_calculate() {
        let a = TT::Years(0);
        let b = TT::Years(1);
        let c = TT::Years(2);

        let d = (a + b + c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Years(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_years_calculate() {
        let a = TT::Years(5);
        let b = TT::Years(3);

        let c = (a - b).calculate();

        assert!(c.is_ok());
        let c = c.unwrap();

        match c {
            TT::Years(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate() {
        let a = TT::Years(3);
        let b = TT::Years(2);
        let c = TT::Years(1);

        let d = (a - b - c).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Years(0) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_addition_of_years_multiple_calculate_reverse_order() {
        let a = TT::Years(0);
        let b = TT::Years(1);
        let c = TT::Years(2);

        let d = (a + (b + c)).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Years(3) => assert!(true),
            _ => assert!(false, "Addition failed"),
        }
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate_reverse_order() {
        let a = TT::Years(3);
        let b = TT::Years(2);
        let c = TT::Years(1);

        let d = (a - (b - c)).calculate();

        assert!(d.is_ok());
        let d = d.unwrap();

        match d {
            TT::Years(2) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

    #[test]
    fn test_subtraction_of_years_multiple_calculate_reverse_order_2() {
        let a = TT::Years(3);
        let b = TT::Years(2);
        let c = TT::Years(1);
        let d = TT::Years(10);

        let e = ((d - c) - (a - b)).calculate();

        assert!(e.is_ok());
        let e = e.unwrap();

        match e {
            TT::Years(8) => assert!(true),
            _ => assert!(false, "Subtraction failed"),
        }
    }

}


