//! The module containing the iterator types
//!

use chrono::NaiveDateTime;

use error::KairosError as KE;
use error::KairosErrorKind as KEK;
use error::Result;
use timetype::TimeType;
use matcher::Matcher;

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
    pub fn skip(&mut self) -> Result<()> {
        self.base += self.increment.clone();
        self.recalculate()
    }

    /// Redo the latest `next()` call with the next `next()` call
    pub fn rollback(&mut self) -> Result<()> {
        self.base -= self.increment.clone();
        self.recalculate()
    }

    fn recalculate(&mut self) -> Result<()> {
        self.base
            .clone()
            .calculate()
            .map(|res| {
                self.base = res;
            })
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
    type Item = Result<TimeType>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.skip().map(|_| self.base.clone()))
    }

}

pub struct FilterIter<I, M>(I, M)
    where I: Iterator<Item = Result<TimeType>>,
          M: Matcher;

impl<I, M> FilterIter<I, M>
    where I: Iterator<Item = Result<TimeType>>,
          M: Matcher
{
    fn new(i: I, m: M) -> FilterIter<I, M> {
        FilterIter(i, m)
    }
}

impl<I, M> Iterator for FilterIter<I, M>
    where I: Iterator<Item = Result<TimeType>>,
          M: Matcher
{
    type Item = Result<TimeType>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None        => return None,
                Some(Err(e)) => return Some(Err(e)),
                Some(Ok(tt)) => match self.1.matches(&tt) {
                    Ok(false) => continue,
                    Ok(true)  => return Some(Ok(tt)),
                    Err(e)    => return Some(Err(e)),
                }
            }
        }
    }
}

pub trait EveryFilter<M: Matcher> : Iterator<Item = Result<TimeType>> + Sized {
    fn every(self, M) -> FilterIter<Self, M>;
}

impl<I, M> EveryFilter<M> for I
    where I: Iterator<Item = Result<TimeType>>,
          M: Matcher
{
    fn every(self, matcher: M) -> FilterIter<Self, M> {
        FilterIter::new(self, matcher)
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

        fn ymd_hms(y: i32, m: u32, d: u32, h: u32, mi: u32, s: u32) -> TT {
            TT::moment(ND::from_ymd(y, m, d).and_hms(h, mi, s))
        }

        #[test]
        fn test_minutely() {
            let minutes = ymd_hms(2000, 1, 1, 0, 0, 0)
                .minutely(1)
                .unwrap()
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

#[cfg(test)]
mod type_tests {
    use super::*;
    use super::extensions::*;

    #[test]
    fn test_iterator_every_once() {
        // This test is solely to check whether this compiles and the API is nice
        let _ = TimeType::today()
            .yearly(1)
            .unwrap()
            .every(::indicator::Day::Monday);
    }

    #[test]
    fn test_iterator_every_twice() {
        // This test is solely to check whether this compiles and the API is nice
        let _ = TimeType::today()
            .yearly(1) // collecting makes us stack-overflow because of the heavy filtering!
            .unwrap()
            .every(::indicator::Day::Monday)
            .every(::indicator::Month::January);
    }
}

#[cfg(all(feature = "with-filters", test))]
mod type_tests_filter_interface {
    use super::*;
    use super::extensions::*;
    use filters::filter::Filter;
    use filters::filter::IntoFilter;

    #[test]
    fn test_compile() {
        // This test is solely to check whether this compiles and the API is nice
        let _ = TimeType::today()
            .daily(1)
            .unwrap()
            .every(::indicator::Day::Monday.into_filter().or(::indicator::Month::January))
            .take(12)
            .collect::<Vec<_>>();
    }
}

