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

use nom::{IResult, space, alpha, alphanumeric, digit};
use std::str;
use std::str::FromStr;

use chrono::NaiveDate;

use timetype;

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

impl Into<timetype::TimeType> for Amount {
    fn into(self) -> timetype::TimeType {
        match self.1 {
            Unit::Second => timetype::TimeType::seconds(self.0),
            Unit::Minute => timetype::TimeType::minutes(self.0),
            Unit::Hour   => timetype::TimeType::hours(self.0),
            Unit::Day    => timetype::TimeType::days(self.0),
            Unit::Week   => timetype::TimeType::weeks(self.0),
            Unit::Month  => timetype::TimeType::months(self.0),
            Unit::Year   => timetype::TimeType::years(self.0),
        }
    }
}

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

impl Into<timetype::TimeType> for AmountExpr {
    fn into(self) -> timetype::TimeType {
        let mut amount = self.amount.into();

        if let Some((op, other_amonut_expr)) = self.next {
            match op {
                Operator::Plus => {
                    amount = amount + (*other_amonut_expr).into();
                },
                Operator::Minus => {
                    amount = amount - (*other_amonut_expr).into();
                },
            }
        }

        amount
    }
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
// The order is relevant here, because datetime is longer than date, we must parse datetime before
// date.
named!(exact_date_parser<ExactDate>, alt_complete!(
    tag!("today")     => { |_| ExactDate::Today } |
    tag!("yesterday") => { |_| ExactDate::Yesterday } |
    tag!("tomorrow")  => { |_| ExactDate::Tomorrow } |
    do_parse!(d: parse_datetime >> (ExactDate::Iso8601DateTime(d))) |
    do_parse!(d: parse_date     >> (ExactDate::Iso8601Date(d)))
));

#[derive(Debug, PartialEq, Eq)]
pub enum ExactDate {
    Today,
    Yesterday,
    Tomorrow,
    Iso8601Date(::iso8601::Date),
    Iso8601DateTime(::iso8601::DateTime)
}

impl Into<timetype::TimeType> for ExactDate {
    fn into(self) -> timetype::TimeType {
        match self {
            ExactDate::Today => timetype::TimeType::today(),
            ExactDate::Yesterday => timetype::TimeType::today() - timetype::TimeType::days(1),
            ExactDate::Tomorrow  => timetype::TimeType::today() + timetype::TimeType::days(1),
            ExactDate::Iso8601Date(date) => {
                let (year, month, day) = match date {
                    ::iso8601::Date::YMD { year, month, day } => {
                        (year, month, day)
                    },
                    ::iso8601::Date::Week { year, ww, d } => {
                        unimplemented!()
                    },
                    ::iso8601::Date::Ordinal { year, ddd } => {
                        unimplemented!()
                    },
                };

                let ndt = NaiveDate::from_ymd(year, month, day).and_hms(0, 0, 0);
                timetype::TimeType::moment(ndt)
            },
            ExactDate::Iso8601DateTime(::iso8601::DateTime { date, time }) => {
                let (hour, minute, second) = (time.hour, time.minute, time.second);
                let (year, month, day) = match date {
                    ::iso8601::Date::YMD { year, month, day } => {
                        (year, month, day)
                    },
                    ::iso8601::Date::Week { year, ww, d } => {
                        unimplemented!()
                    },
                    ::iso8601::Date::Ordinal { year, ddd } => {
                        unimplemented!()
                    },
                };

                let ndt = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second);
                timetype::TimeType::moment(ndt)
            },
        }
    }
}

named!(date<Date>, do_parse!(
    exact:exact_date_parser >>
    o: opt!(do_parse!(sp >> op:operator_parser >> a:amount_expr >> (op, a))) >>
    (Date(exact, o))
));

#[derive(Debug, PartialEq, Eq)]
pub struct Date(ExactDate, Option<(Operator, AmountExpr)>);

impl Into<timetype::TimeType> for Date {
    fn into(self) -> timetype::TimeType {
        let base : timetype::TimeType = self.0.into();
        match self.1 {
            Some((Operator::Plus,  amount)) => base + amount.into(),
            Some((Operator::Minus, amount)) => base - amount.into(),
            None                            => base,
        }
    }
}

/// Main entry function for timetype parser
///
/// # Notice
///
/// Note that this function returns a parser::TimeType, not a timetype::TimeType. Though, the
/// parser::TimeType can be `Into::into()`ed.
///
named!(pub timetype<TimeType>, alt!(
    do_parse!(d: date        >> (TimeType::Date(d))) |
    do_parse!(a: amount_expr >> (TimeType::AmountExpr(a)))
));

#[derive(Debug, PartialEq, Eq)]
pub enum TimeType {
    Date(Date),
    AmountExpr(AmountExpr),
}

impl Into<timetype::TimeType> for TimeType {
    fn into(self) -> timetype::TimeType {
        match self {
            TimeType::Date(d)       => d.into(),
            TimeType::AmountExpr(a) => a.into(),
        }
    }
}

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

impl Into<timetype::TimeType> for Iterator {
    fn into(self) -> timetype::TimeType {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::*;

    use chrono::Timelike;
    use chrono::Datelike;

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

    #[test]
    fn test_parse_expressions_date() {
        use iso8601::Date;
        let res = exact_date_parser(&b"2017-01-01"[..]);
        assert!(res.is_done());

        match res.unwrap().1 {
            ExactDate::Iso8601DateTime(_) => assert!(false),
            ExactDate::Iso8601Date(d) => {
                match d {
                    Date::YMD { year, month, day } => {
                        assert_eq!(year, 2017);
                        assert_eq!(month, 1);
                        assert_eq!(day, 1)
                    },
                    _ => assert!(false),
                }
            },
            ExactDate::Tomorrow       => assert!(false),
            ExactDate::Yesterday      => assert!(false),
            ExactDate::Today          => assert!(false),
        };
    }

    #[test]
    fn test_parse_expressions_datetime() {
        use iso8601::Date;
        let res = exact_date_parser(&b"2017-01-01T22:00:11"[..]);
        assert!(res.is_done());

        match res.unwrap().1 {
            ExactDate::Iso8601DateTime(obj) => {
                match obj.date {
                    Date::YMD { year, month, day } => {
                        assert_eq!(year, 2017);
                        assert_eq!(month, 1);
                        assert_eq!(day, 1)
                    },
                    _ => assert!(false),
                }
                assert_eq!(obj.time.hour, 22);
                assert_eq!(obj.time.minute, 0);
                assert_eq!(obj.time.second, 11);
            },
            ExactDate::Iso8601Date(_) => assert!(false),
            ExactDate::Tomorrow       => assert!(false),
            ExactDate::Yesterday      => assert!(false),
            ExactDate::Today          => assert!(false),
        };
    }

    #[test]
    fn test_expressions_to_date() {
        let res = amount_expr(&b"5min + 12min"[..]);
        assert!(res.is_done());
        let (_, o) = res.unwrap();

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        assert_eq!(calc_res.get_seconds(), 17 * 60);
        assert_eq!(calc_res.get_minutes(), 17);
        assert_eq!(calc_res.get_hours(), 0);
        assert_eq!(calc_res.get_days(), 0);
        assert_eq!(calc_res.get_years(), 0);
    }

    #[test]
    fn test_expressions_to_date_2() {
        let res = amount_expr(&b"5min + 12min + 15hours"[..]);
        assert!(res.is_done());
        let (_, o) = res.unwrap();

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        assert_eq!(calc_res.get_seconds(), 17 * 60 + (15 * 60 * 60));
        assert_eq!(calc_res.get_minutes(), 17 + (15 * 60));
        assert_eq!(calc_res.get_hours(), 15);
        assert_eq!(calc_res.get_days(), 0);
        assert_eq!(calc_res.get_years(), 0);
    }

    #[test]
    fn test_expressions_to_date_3() {
        let res = date(&b"today + 5min + 12min"[..]);
        assert!(res.is_done(), "Not done: {:?}", res.unwrap_err().description());
        let (_, o) = res.unwrap();

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        // because this test is basically dependent on the current time, which is a baaaad use of
        // state in a test, we rely on `test_expressions_to_date_4()` here and assume that the
        // upper assertions are enough.
    }

    #[test]
    fn test_expressions_to_date_4() {
        let res = date(&b"2017-01-01 + 5min + 12min"[..]);
        assert!(res.is_done(), "Not done: {:?}", res.unwrap_err().description());
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        println!("{:#?}", calc_res);

        assert_eq!(calc_res.get_moment().unwrap().year()  , 2017);
        assert_eq!(calc_res.get_moment().unwrap().month() , 01);
        assert_eq!(calc_res.get_moment().unwrap().day()   , 01);
        assert_eq!(calc_res.get_moment().unwrap().hour()  , 00);
        assert_eq!(calc_res.get_moment().unwrap().minute(), 17);
        assert_eq!(calc_res.get_moment().unwrap().second(), 00);
    }

    #[test]
    fn test_expressions_to_timetype() {
        let res = timetype(&b"5min + 12min + 15hours"[..]);
        assert!(res.is_done());
        let (_, o) = res.unwrap();

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        assert_eq!(calc_res.get_seconds(), 17 * 60 + (15 * 60 * 60));
        assert_eq!(calc_res.get_minutes(), 17 + (15 * 60));
        assert_eq!(calc_res.get_hours(), 15);
        assert_eq!(calc_res.get_days(), 0);
        assert_eq!(calc_res.get_years(), 0);
    }

    #[test]
    fn test_expressions_to_timetype_2() {
        let res = timetype(&b"today + 5min + 12min"[..]);
        assert!(res.is_done(), "Not done: {:?}", res.unwrap_err().description());
        let (_, o) = res.unwrap();

        let calc_res : timetype::TimeType = o.into();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        // because this test is basically dependent on the current time, which is a baaaad use of
        // state in a test, we rely on `test_expressions_to_date_4()` here and assume that the
        // upper assertions are enough.
    }

    #[test]
    fn test_expressions_to_timetype_subtract() {
        let res = timetype(&b"5min + 12min + 15hours - 1hour"[..]);
        assert!(res.is_done());
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res : timetype::TimeType = o.into();
        println!("{:#?}", calc_res);

        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());
        println!("{:#?}", calc_res);

        let calc_res = calc_res.unwrap();
        assert_eq!(calc_res.get_seconds(), 17 * 60 + (14 * 60 * 60));
        assert_eq!(calc_res.get_minutes(), 17 + (14 * 60));
        assert_eq!(calc_res.get_hours(), 14);
        assert_eq!(calc_res.get_days(), 0);
        assert_eq!(calc_res.get_years(), 0);
    }

}

