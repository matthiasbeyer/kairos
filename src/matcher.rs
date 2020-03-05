
use chrono::Datelike;

use error::Error;
use error::Result;
use indicator::Day;
use indicator::Month;
use timetype::TimeType;

/// A trait to extend indicator::* to be able to match them with a TimeType object
pub trait Matcher {
    fn matches(&self, tt: &TimeType) -> Result<bool>;
}

impl Matcher for Day {

    fn matches(&self, tt: &TimeType) -> Result<bool> {
        let this : ::chrono::Weekday = self.clone().into();
        tt.get_moment()
            .map(|mom| this == mom.weekday())
            .ok_or(Error::ArgumentErrorNotAMoment(tt.name()))
    }
}

impl Matcher for Month {

    fn matches(&self, tt: &TimeType) -> Result<bool> {
        let this : u32 = self.clone().into();
        tt.get_moment()
            .map(|mom| this == mom.month())
            .ok_or(Error::ArgumentErrorNotAMoment(tt.name()))
    }

}

#[cfg(feature = "with-filters")]
use filters::filter::*;

#[cfg(feature = "with-filters")]
impl<F> Matcher for F
    where F: Filter<TimeType>
{
    fn matches(&self, tt: &TimeType) -> Result<bool> {
        Ok(self.filter(tt))
    }
}

