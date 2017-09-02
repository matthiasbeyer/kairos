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
        (TT::Minutes(a), TT::Minutes(b)) => unimplemented!(),
        (TT::Hours(a), TT::Hours(b))     => unimplemented!(),
        (TT::Days(a), TT::Days(b))       => unimplemented!(),
        (TT::Weeks(a), TT::Weeks(b))     => unimplemented!(),
        (TT::Months(a), TT::Months(b))   => unimplemented!(),
        (TT::Years(a), TT::Years(b))     => unimplemented!(),
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
        (TT::Minutes(a), TT::Minutes(b)) => unimplemented!(),
        (TT::Hours(a), TT::Hours(b))     => unimplemented!(),
        (TT::Days(a), TT::Days(b))       => unimplemented!(),
        (TT::Weeks(a), TT::Weeks(b))     => unimplemented!(),
        (TT::Months(a), TT::Months(b))   => unimplemented!(),
        (TT::Years(a), TT::Years(b))     => unimplemented!(),
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
}

