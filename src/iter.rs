//! The module containing the iterator types
//!

use chrono::NaiveDateTime;

use error::KairosError as KE;
use error::KairosErrorKind as KEK;
use error::Result;
use timetype::TimeType;

pub struct Iter {
    base: TimeType,
    increment: TimeType,
}

// An iterator for creating new TimeType instances based on a base-date plus some increment value
//
// As the iterator only uses TimeType internally, the iterator performs very little computation
// itself. It clones the `increment` one time per `next()` call, thought.
//
// Performing the computation on the yielded `TimeType` instances can be done by transforming this
// iterator into a `CalculatingIter`.
impl Iter {

    pub fn build(base: NaiveDateTime, inc: TimeType) -> Result<Iter> {
        if !inc.is_a_amount() {
            Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(inc)))
        } else {
            Ok(Iter {
                base:       TimeType::moment(base),
                increment:  inc,
            })
        }
    }

    pub fn increment(&self) -> &TimeType {
        &self.increment
    }

    /// Skip one `next()` call
    pub fn skip(&mut self) {
        self.base += self.increment.clone();
    }

    /// Redo the latest `next()` call with the next `next()` call
    pub fn rollback(&mut self) {
        self.base -= self.increment.clone();
    }

}

/// # Warning
///
/// As the iterator does not perform any computation, only wrapping objects into eachother, this is
/// basically an endless iterator.
///
/// As the TimeType internally uses `chrono::Duration` for storing durations, calling this without
/// a bound may result in either a out-of-memory error or a panic because of overflow.
///
/// Be warned.
///
impl Iterator for Iter {
    type Item = TimeType;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip();
        Some(self.base.clone())
    }

}

/// An iterator type that calls `calculate()` on the `TimeType` instances which are yielded by its
/// inner iterator type.
pub struct CalculatingIter<I>(I)
    where I: Iterator<Item = TimeType>;

impl<I> CalculatingIter<I>
    where I: Iterator<Item = TimeType>
{
    pub fn new(i: I) -> CalculatingIter<I> {
        CalculatingIter(i)
    }
}

impl<I> Iterator for CalculatingIter<I>
    where I: Iterator<Item = TimeType>
{
    type Item = Result<TimeType>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(TimeType::calculate)
    }

}

pub trait IntoCalculatingIter : Iterator<Item = TimeType> + Sized {
    fn calculate(self) -> CalculatingIter<Self>;
}

impl<I> IntoCalculatingIter for I
    where I: Iterator<Item = TimeType>
{
    fn calculate(self) -> CalculatingIter<Self> {
        CalculatingIter(self)
    }
}

pub mod extensions {
    use timetype::TimeType as TT;
    use super::Iter;
    use error::Result;
    use error::KairosError as KE;
    use error::KairosErrorKind as KEK;

    pub trait Minutely {
        fn minutely(self, i: i64) -> Result<Iter>;
    }

    pub trait Hourly {
        fn hourly(self, i: i64) -> Result<Iter>;
    }

    pub trait Daily {
        fn daily(self, i: i64) -> Result<Iter>;
    }

    pub trait Weekly : Sized {
        fn weekly(self, i: i64) -> Result<Iter>;
    }

    pub trait Monthly {
        fn monthly(self, i: i64) -> Result<Iter>;
    }

    pub trait Yearly {
        fn yearly(self, i: i64) -> Result<Iter>;
    }

    pub trait Every {
        fn every(self, inc: TT) -> Result<Iter>;
    }

    impl Minutely for TT {

        fn minutely(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::minutes(i);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }

    impl Hourly for TT {

        fn hourly(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::hours(i);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }

    impl Daily for TT {

        fn daily(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::days(i);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }

    impl Weekly for TT {

        /// Conveniance function over `Daily::daily( n * 7 )`
        fn weekly(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::days(i * 7);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }

    impl Monthly for TT {

        fn monthly(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::months(i);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }

    impl Yearly for TT {

        fn yearly(self, i: i64) -> Result<Iter> {
            match self {
                TT::Moment(mom) => {
                    let increment = TT::years(i);
                    assert!(increment.is_a_amount(), "This is a Bug, please report this!");
                    Iter::build(mom, increment)
                },
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }

    }


    impl Every for TT {
        fn every(self, inc: TT) -> Result<Iter> {
            match self {
                TT::Moment(mom) => Iter::build(mom, inc),
                _ => Err(KE::from_kind(KEK::ArgumentErrorNotAnAmount(self))),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use timetype::TimeType as TT;
        use chrono::NaiveDate as ND;
        use iter::IntoCalculatingIter;

        fn ymd_hms(y: i32, m: u32, d: u32, h: u32, mi: u32, s: u32) -> TT {
            TT::moment(ND::from_ymd(y, m, d).and_hms(h, mi, s))
        }

        #[test]
        fn test_minutely() {
            let minutes = ymd_hms(2000, 1, 1, 0, 0, 0)
                .minutely(1)
                .unwrap()
                .calculate()
                .take(5)
                .collect::<Vec<_>>();

            assert_eq!(ymd_hms(2000, 1, 1, 0, 1, 0), *minutes[0].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 0, 2, 0), *minutes[1].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 0, 3, 0), *minutes[2].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 0, 4, 0), *minutes[3].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 0, 5, 0), *minutes[4].as_ref().unwrap());
        }

        #[test]
        fn test_hourly() {
            let minutes = ymd_hms(2000, 1, 1, 0, 0, 0)
                .hourly(1)
                .unwrap()
                .calculate()
                .take(5)
                .collect::<Vec<_>>();

            assert_eq!(ymd_hms(2000, 1, 1, 1, 0, 0), *minutes[0].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 2, 0, 0), *minutes[1].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 3, 0, 0), *minutes[2].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 4, 0, 0), *minutes[3].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1, 1, 5, 0, 0), *minutes[4].as_ref().unwrap());
        }

        #[test]
        fn test_weekly() {
            let minutes = ymd_hms(2000, 1, 1, 1, 0, 0)
                .weekly(1)
                .unwrap()
                .calculate()
                .take(5)
                .collect::<Vec<_>>();

            assert_eq!(ymd_hms(2000, 1, 8, 1, 0, 0), *minutes[0].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1,15, 1, 0, 0), *minutes[1].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1,22, 1, 0, 0), *minutes[2].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 1,29, 1, 0, 0), *minutes[3].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 2, 5, 1, 0, 0), *minutes[4].as_ref().unwrap());
        }

        #[test]
        fn test_monthly() {
            let minutes = ymd_hms(2000, 1, 1, 0, 0, 0)
                .monthly(1)
                .unwrap()
                .calculate()
                .take(5)
                .collect::<Vec<_>>();

            assert_eq!(ymd_hms(2000, 2, 1, 0, 0, 0), *minutes[0].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 3, 1, 0, 0, 0), *minutes[1].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 4, 1, 0, 0, 0), *minutes[2].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 5, 1, 0, 0, 0), *minutes[3].as_ref().unwrap());
            assert_eq!(ymd_hms(2000, 6, 1, 0, 0, 0), *minutes[4].as_ref().unwrap());
        }

        #[test]
        fn test_yearly() {
            let minutes = ymd_hms(2000, 1, 1, 0, 0, 0)
                .yearly(1)
                .unwrap()
                .calculate()
                .take(5)
                .collect::<Vec<_>>();

            assert_eq!(ymd_hms(2001, 1, 1, 0, 0, 0), *minutes[0].as_ref().unwrap());
            assert_eq!(ymd_hms(2002, 1, 1, 0, 0, 0), *minutes[1].as_ref().unwrap());
            assert_eq!(ymd_hms(2003, 1, 1, 0, 0, 0), *minutes[2].as_ref().unwrap());
            assert_eq!(ymd_hms(2004, 1, 1, 0, 0, 0), *minutes[3].as_ref().unwrap());
            assert_eq!(ymd_hms(2005, 1, 1, 0, 0, 0), *minutes[4].as_ref().unwrap());
        }

    }

}
