//! The module for the TimeType
//!

use chrono::NaiveDateTime;

/// A Type of Time, currently based on chrono::NaiveDateTime
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

}

