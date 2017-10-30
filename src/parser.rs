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

use nom::whitespace::sp;

named!(amount_expr_next<(Operator, Box<AmountExpr>)>, do_parse!(
    op:operator_parser
    >> opt!(sp)
    >> amexp:amount_expr
    >> ((op, Box::new(amexp)))
));

named!(amount_expr<AmountExpr>, do_parse!(
    amount:amount_parser >>
    opt!(sp) >>
    o: opt!(complete!(amount_expr_next)) >>
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

#[derive(Debug, PartialEq, Eq)]
pub struct Iterator(Date, Iterspec, Option<UntilSpec>);


#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(integer(&b"2"[..]), IResult::Done(&b""[..], 2));
        assert_eq!(integer(&b"217"[..]), IResult::Done(&b""[..], 217));
    }

    #[test]
    fn test_unit() {
        assert_eq!(unit_parser(&b"second"[..]), IResult::Done(&b""[..], Unit::Second));
        assert_eq!(unit_parser(&b"seconds"[..]), IResult::Done(&b""[..], Unit::Second));
        assert_eq!(unit_parser(&b"sec"[..]), IResult::Done(&b""[..], Unit::Second));
        assert_eq!(unit_parser(&b"secs"[..]), IResult::Done(&b""[..], Unit::Second));
        assert_eq!(unit_parser(&b"s"[..]), IResult::Done(&b""[..], Unit::Second));
        assert_eq!(unit_parser(&b"minute"[..]), IResult::Done(&b""[..], Unit::Minute));
        assert_eq!(unit_parser(&b"minutes"[..]), IResult::Done(&b""[..], Unit::Minute));
        assert_eq!(unit_parser(&b"min"[..]), IResult::Done(&b""[..], Unit::Minute));
        assert_eq!(unit_parser(&b"mins"[..]), IResult::Done(&b""[..], Unit::Minute));
        assert_eq!(unit_parser(&b"hour"[..]), IResult::Done(&b""[..], Unit::Hour));
        assert_eq!(unit_parser(&b"hours"[..]), IResult::Done(&b""[..], Unit::Hour));
        assert_eq!(unit_parser(&b"hr"[..]), IResult::Done(&b""[..], Unit::Hour));
        assert_eq!(unit_parser(&b"hrs"[..]), IResult::Done(&b""[..], Unit::Hour));
        assert_eq!(unit_parser(&b"day"[..]), IResult::Done(&b""[..], Unit::Day));
        assert_eq!(unit_parser(&b"days"[..]), IResult::Done(&b""[..], Unit::Day));
        assert_eq!(unit_parser(&b"d"[..]), IResult::Done(&b""[..], Unit::Day));
        assert_eq!(unit_parser(&b"week"[..]), IResult::Done(&b""[..], Unit::Week));
        assert_eq!(unit_parser(&b"weeks"[..]), IResult::Done(&b""[..], Unit::Week));
        assert_eq!(unit_parser(&b"w"[..]), IResult::Done(&b""[..], Unit::Week));
        assert_eq!(unit_parser(&b"month"[..]), IResult::Done(&b""[..], Unit::Month));
        assert_eq!(unit_parser(&b"months"[..]), IResult::Done(&b""[..], Unit::Month));
        assert_eq!(unit_parser(&b"year"[..]), IResult::Done(&b""[..], Unit::Year));
        assert_eq!(unit_parser(&b"years"[..]), IResult::Done(&b""[..], Unit::Year));
        assert_eq!(unit_parser(&b"yrs"[..]), IResult::Done(&b""[..], Unit::Year));
    }

    #[test]
    fn test_operator() {
        assert_eq!(operator_parser(&b"+"[..]), IResult::Done(&b""[..], Operator::Plus));
        assert_eq!(operator_parser(&b"-"[..]), IResult::Done(&b""[..], Operator::Minus));
    }

    #[test]
    fn test_amount() {
        assert_eq!(amount_parser(&b"5s"[..]), IResult::Done(&b""[..], Amount(5, Unit::Second)));
        assert_eq!(amount_parser(&b"5min"[..]), IResult::Done(&b""[..], Amount(5, Unit::Minute)));
        assert_eq!(amount_parser(&b"55hrs"[..]), IResult::Done(&b""[..], Amount(55, Unit::Hour)));
        assert_eq!(amount_parser(&b"25days"[..]), IResult::Done(&b""[..], Amount(25, Unit::Day)));
        assert_eq!(amount_parser(&b"15weeks"[..]), IResult::Done(&b""[..], Amount(15, Unit::Week)));
    }

    #[test]
    fn test_iterspec() {
        assert_eq!(iter_spec(&b"secondly"[..]), IResult::Done(&b""[..], Iterspec::Secondly));
        assert_eq!(iter_spec(&b"minutely"[..]), IResult::Done(&b""[..], Iterspec::Minutely));
        assert_eq!(iter_spec(&b"hourly"[..]), IResult::Done(&b""[..], Iterspec::Hourly));
        assert_eq!(iter_spec(&b"daily"[..]), IResult::Done(&b""[..], Iterspec::Daily));
        assert_eq!(iter_spec(&b"weekly"[..]), IResult::Done(&b""[..], Iterspec::Weekly));
        assert_eq!(iter_spec(&b"monthly"[..]), IResult::Done(&b""[..], Iterspec::Monthly));
        assert_eq!(iter_spec(&b"yearly"[..]), IResult::Done(&b""[..], Iterspec::Yearly));
        assert_eq!(iter_spec(&b"every 5min"[..]), IResult::Done(&b""[..], Iterspec::Every(5, Unit::Minute)));
    }

    #[test]
    fn test_amountexpr_next() {
        assert_eq!(amount_expr_next(&b"+ 12minutes"[..]),
            IResult::Done(&b""[..],
                (
                    Operator::Plus,
                    Box::new(AmountExpr { amount: Amount(12, Unit::Minute), next: None })
                )
        ));
    }

    #[test]
    fn test_amountexpr() {
        assert_eq!(amount_expr(&b"5minutes"[..]),
            IResult::Done(&b""[..],
                          AmountExpr {
                              amount: Amount(5, Unit::Minute),
                              next: None
                          })
        );

        assert_eq!(amount_expr(&b"5min + 12min"[..]),
        IResult::Done(&b""[..],
                      AmountExpr {
                          amount: Amount(5, Unit::Minute),
                          next: Some((Operator::Plus, Box::new(
                                      AmountExpr {
                                          amount: Amount(12, Unit::Minute),
                                          next: None
                                      })))
                      }));
    }
}

