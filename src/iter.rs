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

    pub fn calculate(self) -> CalculatingIter<Self> {
        CalculatingIter::new(self)
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

impl<I: Iterator<Item = TimeType>> CalculatingIter<I> {
    pub fn new(i: I) -> CalculatingIter<I> {
        CalculatingIter(i)
    }
}

impl<I: Iterator<Item = TimeType>> Iterator for CalculatingIter<I> {
    type Item = Result<TimeType>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(TimeType::calculate)
    }

}

