//! The definition of the "kairos" syntax, for parsing user input into TimeType objects
//!
//! The syntax itself is described in the grammar.rustpeg file.
//! Here goes a documentation on the syntax
//!
//! # Syntax
//!
//! ## Units
//!
//! UnitSec   = "second" | "seconds" | "sec" | "secs" | "s"
//! UnitMin   = "minute" | "minutes" | "min" | "mins"
//! UnitHr    = "hour"   | "hours"   | "hr" | "hrs"
//! UnitDay   = "day"    | "days"    | "d"
//! UnitWeek  = "week"   | "weeks"   | "w"
//! UnitMonth = "month"  | "months"  |
//! UnitYear  = "year"   | "years"   | "yrs"
//! Unit      = UnitSec | UnitMin | UnitHr | UnitDay | UnitWeek | UnitMonth | UnitYear
//!
//! ## Operators
//!
//! Operator  = "+" | "-"
//!
//! ## Intermediate syntax nodes
//!
//! Amount    = "<Number><Unit>"
//!
//! TextIterSpec = "secondly" | "minutely" | "hourly" | "daily" | "weekly" | "monthly" | "yearly"
//! Iterspec     = TextIterSpec | "every" <Number><Unit>
//!
//! ## User-facing syntax nodes
//!
//! AmountExpr = <Amount> (<Operator> <AmountExpr>)?
//! ExactDate  = "today" | "yesterday" | "tomorrow" | <Iso8601>
//! Date       = <ExactDate> (<Operator> <AmountExpr>)?
//! Iterator   = <Date> <Iterspec> ("until" <ExactDate> | <number> "times")?
//!
//! # Warning
//!
//! This module is not intended for public use... it is still public, so you can use it, but you
//! should know that these interfaces are considered private and I will not follow semver and
//! update the minor or major semver numbers of the interface of this module changes.
//!
//! Be warned!
//!

use nom::branch::alt;
use nom::combinator::{complete, map};
use nom::IResult;

use iterator::iterator;
use timetype::timetype;

use crate::error::Error;
use crate::error::Result;
use crate::iter::Iter;
use crate::timetype::IntoTimeType;

mod iterator;
mod timetype;

pub enum Parsed {
    Iterator(Result<iterator::UserIterator<Iter>>),
    TimeType(crate::timetype::TimeType),
}

fn do_parse(input: &[u8]) -> IResult<&[u8], Result<Parsed>> {
    complete(alt((
        map(iterator, |it| Ok(Parsed::Iterator(it.into_user_iterator()))),
        map(timetype, |tt| tt.into_timetype().map(Parsed::TimeType)),
    )))(input)
}

pub fn parse(s: &str) -> Result<Parsed> {
    match do_parse(s.as_bytes()) {
        Ok((_, Ok(o))) => Ok(o),
        Ok((_, Err(e))) => Err(e),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(Error::NomError(e.code.description().to_string())),
        Err(nom::Err::Incomplete(_)) => Err(Error::UnknownParserError),
    }
}
