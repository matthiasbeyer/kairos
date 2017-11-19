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

use nom::Needed;
use nom::IResult;

mod timetype;
mod iterator;

use error::Result;
use error::KairosErrorKind as KEK;
use iter::Iter;
use parser::timetype::timetype;
use parser::iterator::iterator;

pub enum Parsed {
    Iterator(Result<::parser::iterator::UserIterator<Iter>>),
    TimeType(::timetype::TimeType)
}

named!(do_parse<Parsed>, alt_complete!(
    do_parse!(it: iterator >> (Parsed::Iterator(it.into_user_iterator()))) |
    do_parse!(tt: timetype >> (Parsed::TimeType(tt.into())))
));

pub fn parse(s: &str) -> Result<Parsed> {
    match do_parse(s.as_bytes()) {
        IResult::Done(_, o)                  => Ok(o),
        IResult::Error(e)               => Err(e).map_err(From::from),
        IResult::Incomplete(Needed::Unknown) => Err(KEK::UnknownParserError.into()),
        IResult::Incomplete(Needed::Size(s)) => Err(KEK::UnknownParserError.into()),

    }
}

