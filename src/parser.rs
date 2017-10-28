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

use nom::{IResult, space, alpha, alphanumeric, digit};
use std::str;
use std::str::FromStr;

named!(integer<i64>, alt!(
    map_res!(
        map_res!(
            ws!(digit),
            str::from_utf8
        ),
        FromStr::from_str
    )
));

// WARNING: Order is important here. Long tags first, shorter tags later
named!(unit_parser<Unit>, alt_complete!(
    tag!("seconds") => { |_| Unit::Second } |
    tag!("second")  => { |_| Unit::Second } |
    tag!("secs")    => { |_| Unit::Second } |
    tag!("sec")     => { |_| Unit::Second } |
    tag!("s")       => { |_| Unit::Second } |
    tag!("minutes") => { |_| Unit::Minute } |
    tag!("minute")  => { |_| Unit::Minute } |
    tag!("mins")    => { |_| Unit::Minute } |
    tag!("min")     => { |_| Unit::Minute } |
    tag!("hours")   => { |_| Unit::Hour } |
    tag!("hour")    => { |_| Unit::Hour } |
    tag!("hrs")     => { |_| Unit::Hour } |
    tag!("hr")      => { |_| Unit::Hour } |
    tag!("days")    => { |_| Unit::Day } |
    tag!("day")     => { |_| Unit::Day } |
    tag!("d")       => { |_| Unit::Day } |
    tag!("weeks")   => { |_| Unit::Week } |
    tag!("week")    => { |_| Unit::Week } |
    tag!("w")       => { |_| Unit::Week } |
    tag!("months")  => { |_| Unit::Month } |
    tag!("month")   => { |_| Unit::Month } |
    tag!("years")   => { |_| Unit::Year } |
    tag!("year")    => { |_| Unit::Year } |
    tag!("yrs")     => { |_| Unit::Year }
));

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

named!(operator_parser<Operator>, alt!(
    tag!("+") => { |_| Operator::Plus } |
    tag!("-") => { |_| Operator::Minus }
));

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
}

named!(amount_parser<Amount>, do_parse!(
    number: integer >>
    unit : unit_parser >>
    (Amount(number, unit))
));

#[derive(Debug, PartialEq, Eq)]
pub struct Amount(i64, Unit);

named!(iter_spec<Iterspec>, alt_complete!(
    tag!("secondly") => { |_| Iterspec::Secondly } |
    tag!("minutely") => { |_| Iterspec::Minutely } |
    tag!("hourly")   => { |_| Iterspec::Hourly } |
    tag!("daily")    => { |_| Iterspec::Daily } |
    tag!("weekly")   => { |_| Iterspec::Weekly } |
    tag!("monthly")  => { |_| Iterspec::Monthly } |
    tag!("yearly")   => { |_| Iterspec::Yearly } |
    do_parse!(
        tag!("every") >>
        number:integer >>
        unit:unit_parser >>
        (Iterspec::Every(number, unit))
    )
));

#[derive(Debug, PartialEq, Eq)]
pub enum Iterspec {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Every(i64, Unit),
}

named!(amount_expr<AmountExpr>, do_parse!(
    amount:amount_parser >>
    o: opt!(do_parse!(op:operator_parser >> amexp:amount_expr >> ((op, Box::new(amexp))))) >>
    (AmountExpr { amount: amount, next: o, })
));

#[derive(Debug, PartialEq, Eq)]
pub struct AmountExpr {
    amount: Amount,
    next: Option<(Operator, Box<AmountExpr>)>,
}

impl AmountExpr {
    fn new(amount: Amount, next: Option<(Operator, Box<AmountExpr>)>) -> AmountExpr {
        AmountExpr {
            amount: amount,
            next: next
        }
    }
}

use iso8601::parsers::parse_date;
use iso8601::parsers::parse_datetime;
named!(exact_date_parser<ExactDate>, alt!(
    tag!("today")     => { |_| ExactDate::Today } |
    tag!("yesterday") => { |_| ExactDate::Yesterday } |
    tag!("tomorrow")  => { |_| ExactDate::Tomorrow } |
    do_parse!(d: parse_date     >> (ExactDate::Iso8601Date(d))) |
    do_parse!(d: parse_datetime >> (ExactDate::Iso8601DateTime(d)))
));

#[derive(Debug, PartialEq, Eq)]
pub enum ExactDate {
    Today,
    Yesterday,
    Tomorrow,
    Iso8601Date(::iso8601::Date),
    Iso8601DateTime(::iso8601::DateTime)
}

named!(date<Date>, do_parse!(
    exact:exact_date_parser >>
    o: opt!(do_parse!(op:operator_parser >> a:amount_expr >> (op, a))) >>
    (Date(exact, o))
));

#[derive(Debug, PartialEq, Eq)]
pub struct Date(ExactDate, Option<(Operator, AmountExpr)>);

named!(until_spec<UntilSpec>, alt!(
    do_parse!(
        tag!("until") >>
        exact: exact_date_parser >>
        (UntilSpec::Exact(exact))
    ) |
    do_parse!(
        num: integer >>
        tag!("times") >>
        (UntilSpec::Times(num))
    )
));

#[derive(Debug, PartialEq, Eq)]
pub enum UntilSpec {
    Exact(ExactDate),
    Times(i64)
}

named!(iterator<Iterator>, do_parse!(
    d: date                 >>
    spec: iter_spec         >>
    until: opt!(until_spec) >>
    (Iterator(d, spec, until))
));

pub struct Iterator(Date, Iterspec, Option<UntilSpec>);

