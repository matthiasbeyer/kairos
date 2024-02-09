use std::str;

use chrono::NaiveDate;
use iso8601::parsers::{parse_date, parse_datetime};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::combinator::{complete, map, map_opt, opt};
use nom::sequence::{delimited, tuple};
use nom::IResult;

use crate::error::Error;
use crate::error::Result;
use crate::timetype::IntoTimeType;

pub fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    map_opt(delimited(multispace0, digit1, multispace0), |digit| {
        str::from_utf8(digit).ok().and_then(|s| s.parse().ok())
    })(input)
}

// WARNING: Order is important here. Long tags first, shorter tags later
pub fn unit_parser(input: &[u8]) -> IResult<&[u8], Unit> {
    complete(alt((
        map(
            alt((tag("seconds"), tag("second"), tag("secs"), tag("sec"), tag("s"))),
            |_| Unit::Second,
        ),
        map(alt((tag("minutes"), tag("minute"), tag("mins"), tag("min"))), |_| {
            Unit::Minute
        }),
        map(alt((tag("hours"), tag("hour"), tag("hrs"), tag("hr"))), |_| Unit::Hour),
        map(alt((tag("days"), tag("day"), tag("d"))), |_| Unit::Day),
        map(alt((tag("weeks"), tag("week"), tag("w"))), |_| Unit::Week),
        map(alt((tag("months"), tag("month"))), |_| Unit::Month),
        map(alt((tag("years"), tag("year"), tag("yrs"))), |_| Unit::Year),
    )))(input)
}

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

#[derive(Debug, PartialEq, Eq)]
pub enum UnitAlias {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl From<UnitAlias> for Unit {
    fn from(val: UnitAlias) -> Self {
        match val {
            UnitAlias::Secondly => Unit::Second,
            UnitAlias::Minutely => Unit::Minute,
            UnitAlias::Hourly => Unit::Hour,
            UnitAlias::Daily => Unit::Day,
            UnitAlias::Weekly => Unit::Week,
            UnitAlias::Monthly => Unit::Month,
            UnitAlias::Yearly => Unit::Year,
        }
    }
}

pub fn operator_parser(input: &[u8]) -> IResult<&[u8], Operator> {
    alt((map(tag("+"), |_| Operator::Plus), map(tag("-"), |_| Operator::Minus)))(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
}

pub fn unit_alias(input: &[u8]) -> IResult<&[u8], UnitAlias> {
    complete(alt((
        map(tag("secondly"), |_| UnitAlias::Secondly),
        map(tag("minutely"), |_| UnitAlias::Minutely),
        map(tag("hourly"), |_| UnitAlias::Hourly),
        map(tag("daily"), |_| UnitAlias::Daily),
        map(tag("weekly"), |_| UnitAlias::Weekly),
        map(tag("monthly"), |_| UnitAlias::Monthly),
        map(tag("yearly"), |_| UnitAlias::Yearly),
    )))(input)
}

pub fn amount_parser(input: &[u8]) -> IResult<&[u8], Amount> {
    alt((
        map(tuple((integer, unit_parser)), |(number, unit)| Amount(number, unit)),
        map(unit_alias, |unit| Amount(1, unit.into())),
    ))(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Amount(i64, Unit);

impl IntoTimeType for Amount {
    fn into_timetype(self) -> Result<crate::timetype::TimeType> {
        Ok(match self.1 {
            Unit::Second => crate::timetype::TimeType::seconds(self.0),
            Unit::Minute => crate::timetype::TimeType::minutes(self.0),
            Unit::Hour => crate::timetype::TimeType::hours(self.0),
            Unit::Day => crate::timetype::TimeType::days(self.0),
            Unit::Week => crate::timetype::TimeType::weeks(self.0),
            Unit::Month => crate::timetype::TimeType::months(self.0),
            Unit::Year => crate::timetype::TimeType::years(self.0),
        })
    }
}

pub fn amount_expr_next(input: &[u8]) -> IResult<&[u8], (Operator, Box<AmountExpr>)> {
    map(tuple((operator_parser, multispace0, amount_expr)), |(op, _, amexp)| {
        (op, Box::new(amexp))
    })(input)
}

pub fn amount_expr(input: &[u8]) -> IResult<&[u8], AmountExpr> {
    map(
        tuple((amount_parser, multispace0, opt(complete(amount_expr_next)))),
        |(amount, _, next)| AmountExpr { amount, next },
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct AmountExpr {
    amount: Amount,
    next: Option<(Operator, Box<AmountExpr>)>,
}

impl IntoTimeType for AmountExpr {
    fn into_timetype(self) -> Result<crate::timetype::TimeType> {
        let mut amount = self.amount.into_timetype()?;

        if let Some((op, other_amonut_expr)) = self.next {
            match op {
                Operator::Plus => amount += (*other_amonut_expr).into_timetype()?,
                Operator::Minus => amount -= (*other_amonut_expr).into_timetype()?,
            }
        }

        Ok(amount)
    }
}

// The order is relevant here, because datetime is longer than date, we must parse datetime before
// date.
pub fn exact_date_parser(input: &[u8]) -> IResult<&[u8], ExactDate> {
    complete(alt((
        map(tag("today"), |_| ExactDate::Today),
        map(tag("yesterday"), |_| ExactDate::Yesterday),
        map(tag("tomorrow"), |_| ExactDate::Tomorrow),
        map(parse_datetime, ExactDate::Iso8601DateTime),
        map(parse_date, ExactDate::Iso8601Date),
    )))(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExactDate {
    Today,
    Yesterday,
    Tomorrow,
    Iso8601Date(iso8601::Date),
    Iso8601DateTime(iso8601::DateTime),
}

impl IntoTimeType for ExactDate {
    fn into_timetype(self) -> Result<crate::timetype::TimeType> {
        match self {
            ExactDate::Today => Ok(crate::timetype::TimeType::today()),
            ExactDate::Yesterday => Ok(crate::timetype::TimeType::today() - crate::timetype::TimeType::days(1)),
            ExactDate::Tomorrow => Ok(crate::timetype::TimeType::today() + crate::timetype::TimeType::days(1)),
            ExactDate::Iso8601Date(date) => match date {
                iso8601::Date::YMD { year, month, day } => NaiveDate::from_ymd_opt(year, month, day)
                    .and_then(|ndt| ndt.and_hms_opt(0, 0, 0))
                    .ok_or(Error::OutOfBounds(year, month, day, 0, 0, 0))
                    .map(crate::timetype::TimeType::moment),

                iso8601::Date::Week { year, ww, d } => NaiveDate::from_ymd_opt(year, 1, 1)
                    .and_then(|ndt| ndt.and_hms_opt(0, 0, 0))
                    .ok_or(Error::OutOfBounds(year, 1, 1, 0, 0, 0))
                    .map(crate::timetype::TimeType::moment)
                    .map(|m| {
                        m + crate::timetype::TimeType::weeks(ww as i64) + crate::timetype::TimeType::days(d as i64)
                    }),

                iso8601::Date::Ordinal { year, ddd } => NaiveDate::from_ymd_opt(year, 1, 1)
                    .and_then(|ndt| ndt.and_hms_opt(0, 0, 0))
                    .ok_or(Error::OutOfBounds(year, 1, 1, 0, 0, 0))
                    .map(crate::timetype::TimeType::moment)
                    .map(|m| m + crate::timetype::TimeType::days(ddd as i64)),
            },
            ExactDate::Iso8601DateTime(iso8601::DateTime { date, time }) => {
                let (hour, minute, second) = (time.hour, time.minute, time.second);

                match date {
                    iso8601::Date::YMD { year, month, day } => NaiveDate::from_ymd_opt(year, month, day)
                        .and_then(|ndt| ndt.and_hms_opt(hour, minute, second))
                        .ok_or(Error::OutOfBounds(year, month, day, hour, minute, second))
                        .map(crate::timetype::TimeType::moment),

                    iso8601::Date::Week { year, ww, d } => NaiveDate::from_ymd_opt(year, 1, 1)
                        .and_then(|ndt| ndt.and_hms_opt(0, 0, 0))
                        .ok_or(Error::OutOfBounds(year, 1, 1, 0, 0, 0))
                        .map(crate::timetype::TimeType::moment)
                        .map(|m| {
                            m + crate::timetype::TimeType::weeks(ww as i64)
                                + crate::timetype::TimeType::days(d as i64)
                                + crate::timetype::TimeType::hours(hour as i64)
                                + crate::timetype::TimeType::minutes(minute as i64)
                                + crate::timetype::TimeType::seconds(second as i64)
                        }),

                    iso8601::Date::Ordinal { year, ddd } => NaiveDate::from_ymd_opt(year, 1, 1)
                        .and_then(|ndt| ndt.and_hms_opt(0, 0, 0))
                        .ok_or(Error::OutOfBounds(year, 1, 1, 0, 0, 0))
                        .map(crate::timetype::TimeType::moment)
                        .map(|m| {
                            m + crate::timetype::TimeType::days(ddd as i64)
                                + crate::timetype::TimeType::hours(hour as i64)
                                + crate::timetype::TimeType::minutes(minute as i64)
                                + crate::timetype::TimeType::seconds(second as i64)
                        }),
                }
            },
        }
    }
}

pub fn date(input: &[u8]) -> IResult<&[u8], Date> {
    map(
        tuple((
            exact_date_parser,
            opt(complete(map(
                tuple((multispace1, operator_parser, multispace1, amount_expr)),
                |(_, op, _, a)| (op, a),
            ))),
        )),
        |(exact, o)| Date(exact, o),
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Date(ExactDate, Option<(Operator, AmountExpr)>);

impl IntoTimeType for Date {
    fn into_timetype(self) -> Result<crate::timetype::TimeType> {
        let base: crate::timetype::TimeType = self.0.into_timetype()?;
        match self.1 {
            Some((Operator::Plus, amount)) => Ok(base + amount.into_timetype()?),
            Some((Operator::Minus, amount)) => Ok(base - amount.into_timetype()?),
            None => Ok(base),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeType {
    Date(Date),
    AmountExpr(AmountExpr),
}

impl IntoTimeType for TimeType {
    fn into_timetype(self) -> Result<crate::timetype::TimeType> {
        match self {
            TimeType::Date(d) => d.into_timetype(),
            TimeType::AmountExpr(a) => a.into_timetype(),
        }
    }
}

// Main entry function for timetype parser
//
// # Notice
//
// Note that this function returns a parser::TimeType, not a timetype::TimeType. Though, the
// parser::TimeType can be `Into::into()`ed.
//
pub fn timetype(input: &[u8]) -> IResult<&[u8], TimeType> {
    alt((map(date, TimeType::Date), map(amount_expr, TimeType::AmountExpr)))(input)
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use chrono::Timelike;

    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(integer(&b"2"[..]), Ok((&b""[..], 2)));
        assert_eq!(integer(&b"217"[..]), Ok((&b""[..], 217)));
    }

    #[test]
    fn test_unit() {
        assert_eq!(unit_parser(&b"second"[..]), Ok((&b""[..], Unit::Second)));
        assert_eq!(unit_parser(&b"seconds"[..]), Ok((&b""[..], Unit::Second)));
        assert_eq!(unit_parser(&b"sec"[..]), Ok((&b""[..], Unit::Second)));
        assert_eq!(unit_parser(&b"secs"[..]), Ok((&b""[..], Unit::Second)));
        assert_eq!(unit_parser(&b"s"[..]), Ok((&b""[..], Unit::Second)));
        assert_eq!(unit_parser(&b"minute"[..]), Ok((&b""[..], Unit::Minute)));
        assert_eq!(unit_parser(&b"minutes"[..]), Ok((&b""[..], Unit::Minute)));
        assert_eq!(unit_parser(&b"min"[..]), Ok((&b""[..], Unit::Minute)));
        assert_eq!(unit_parser(&b"mins"[..]), Ok((&b""[..], Unit::Minute)));
        assert_eq!(unit_parser(&b"hour"[..]), Ok((&b""[..], Unit::Hour)));
        assert_eq!(unit_parser(&b"hours"[..]), Ok((&b""[..], Unit::Hour)));
        assert_eq!(unit_parser(&b"hr"[..]), Ok((&b""[..], Unit::Hour)));
        assert_eq!(unit_parser(&b"hrs"[..]), Ok((&b""[..], Unit::Hour)));
        assert_eq!(unit_parser(&b"day"[..]), Ok((&b""[..], Unit::Day)));
        assert_eq!(unit_parser(&b"days"[..]), Ok((&b""[..], Unit::Day)));
        assert_eq!(unit_parser(&b"d"[..]), Ok((&b""[..], Unit::Day)));
        assert_eq!(unit_parser(&b"week"[..]), Ok((&b""[..], Unit::Week)));
        assert_eq!(unit_parser(&b"weeks"[..]), Ok((&b""[..], Unit::Week)));
        assert_eq!(unit_parser(&b"w"[..]), Ok((&b""[..], Unit::Week)));
        assert_eq!(unit_parser(&b"month"[..]), Ok((&b""[..], Unit::Month)));
        assert_eq!(unit_parser(&b"months"[..]), Ok((&b""[..], Unit::Month)));
        assert_eq!(unit_parser(&b"year"[..]), Ok((&b""[..], Unit::Year)));
        assert_eq!(unit_parser(&b"years"[..]), Ok((&b""[..], Unit::Year)));
        assert_eq!(unit_parser(&b"yrs"[..]), Ok((&b""[..], Unit::Year)));
    }

    #[test]
    fn test_unit_alias() {
        assert_eq!(unit_alias(&b"secondly"[..]), Ok((&b""[..], UnitAlias::Secondly)));
        assert_eq!(unit_alias(&b"minutely"[..]), Ok((&b""[..], UnitAlias::Minutely)));
        assert_eq!(unit_alias(&b"hourly"[..]), Ok((&b""[..], UnitAlias::Hourly)));
        assert_eq!(unit_alias(&b"daily"[..]), Ok((&b""[..], UnitAlias::Daily)));
        assert_eq!(unit_alias(&b"weekly"[..]), Ok((&b""[..], UnitAlias::Weekly)));
        assert_eq!(unit_alias(&b"monthly"[..]), Ok((&b""[..], UnitAlias::Monthly)));
        assert_eq!(unit_alias(&b"yearly"[..]), Ok((&b""[..], UnitAlias::Yearly)));
    }

    #[test]
    fn test_operator() {
        assert_eq!(operator_parser(&b"+"[..]), Ok((&b""[..], Operator::Plus)));
        assert_eq!(operator_parser(&b"-"[..]), Ok((&b""[..], Operator::Minus)));
    }

    #[test]
    fn test_amount() {
        assert_eq!(amount_parser(&b"5s"[..]), Ok((&b""[..], Amount(5, Unit::Second))));
        assert_eq!(amount_parser(&b"5min"[..]), Ok((&b""[..], Amount(5, Unit::Minute))));
        assert_eq!(amount_parser(&b"55hrs"[..]), Ok((&b""[..], Amount(55, Unit::Hour))));
        assert_eq!(amount_parser(&b"25days"[..]), Ok((&b""[..], Amount(25, Unit::Day))));
        assert_eq!(amount_parser(&b"15weeks"[..]), Ok((&b""[..], Amount(15, Unit::Week))));
    }

    #[test]
    fn test_unit_alias_with_amount_parser() {
        assert_eq!(amount_parser(&b"secondly"[..]), Ok((&b""[..], Amount(1, Unit::Second))));
        assert_eq!(amount_parser(&b"minutely"[..]), Ok((&b""[..], Amount(1, Unit::Minute))));
        assert_eq!(amount_parser(&b"hourly"[..]), Ok((&b""[..], Amount(1, Unit::Hour))));
        assert_eq!(amount_parser(&b"daily"[..]), Ok((&b""[..], Amount(1, Unit::Day))));
        assert_eq!(amount_parser(&b"weekly"[..]), Ok((&b""[..], Amount(1, Unit::Week))));
        assert_eq!(amount_parser(&b"monthly"[..]), Ok((&b""[..], Amount(1, Unit::Month))));
        assert_eq!(amount_parser(&b"yearly"[..]), Ok((&b""[..], Amount(1, Unit::Year))));
    }

    #[test]
    fn test_amountexpr_next() {
        assert_eq!(
            amount_expr_next(&b"+ 12minutes"[..]),
            Ok((
                &b""[..],
                (
                    Operator::Plus,
                    Box::new(AmountExpr {
                        amount: Amount(12, Unit::Minute),
                        next: None
                    })
                )
            ))
        );
    }

    #[test]
    fn test_amountexpr() {
        assert_eq!(
            amount_expr(&b"5minutes"[..]),
            Ok((
                &b""[..],
                AmountExpr {
                    amount: Amount(5, Unit::Minute),
                    next: None
                }
            ))
        );

        assert_eq!(
            amount_expr(&b"5min + 12min"[..]),
            Ok((
                &b""[..],
                AmountExpr {
                    amount: Amount(5, Unit::Minute),
                    next: Some((
                        Operator::Plus,
                        Box::new(AmountExpr {
                            amount: Amount(12, Unit::Minute),
                            next: None
                        })
                    ))
                }
            ))
        );
    }

    #[test]
    fn test_parse_expressions_date() {
        use iso8601::Date;
        let res = exact_date_parser(&b"2017-01-01"[..]);
        assert!(res.is_ok());

        match res.unwrap().1 {
            ExactDate::Iso8601DateTime(_) => panic!("Unexpected enum variant"),
            ExactDate::Iso8601Date(d) => match d {
                Date::YMD { year, month, day } => {
                    assert_eq!(year, 2017);
                    assert_eq!(month, 1);
                    assert_eq!(day, 1)
                },
                _ => panic!("Unexpected enum variant"),
            },
            ExactDate::Tomorrow => panic!("Unexpected enum variant"),
            ExactDate::Yesterday => panic!("Unexpected enum variant"),
            ExactDate::Today => panic!("Unexpected enum variant"),
        };
    }

    #[test]
    fn test_parse_expressions_datetime() {
        use iso8601::Date;
        let res = exact_date_parser(&b"2017-01-01T22:00:11"[..]);
        assert!(res.is_ok());

        match res.unwrap().1 {
            ExactDate::Iso8601DateTime(obj) => {
                match obj.date {
                    Date::YMD { year, month, day } => {
                        assert_eq!(year, 2017);
                        assert_eq!(month, 1);
                        assert_eq!(day, 1)
                    },
                    _ => panic!("Unexpected enum variant"),
                }
                assert_eq!(obj.time.hour, 22);
                assert_eq!(obj.time.minute, 0);
                assert_eq!(obj.time.second, 11);
            },
            ExactDate::Iso8601Date(_) => panic!("Unexpected enum variant"),
            ExactDate::Tomorrow => panic!("Unexpected enum variant"),
            ExactDate::Yesterday => panic!("Unexpected enum variant"),
            ExactDate::Today => panic!("Unexpected enum variant"),
        };
    }

    #[test]
    fn test_simple_date_1() {
        let res = exact_date_parser(&b"today"[..]);
        assert!(res.is_ok(), "Not done: {:?}", res);

        let res = date(&b"today"[..]);
        assert!(res.is_ok(), "Not done: {:?}", res);
    }

    #[test]
    fn test_simple_date_2() {
        let res = date(&b"2017-01-01"[..]);
        assert!(res.is_ok(), "Not done: {:?}", res);
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        println!("{:#?}", calc_res);

        assert_eq!(calc_res.get_moment().unwrap().year(), 2017);
        assert_eq!(calc_res.get_moment().unwrap().month(), 1);
        assert_eq!(calc_res.get_moment().unwrap().day(), 1);
        assert_eq!(calc_res.get_moment().unwrap().hour(), 0);
        assert_eq!(calc_res.get_moment().unwrap().minute(), 0);
        assert_eq!(calc_res.get_moment().unwrap().second(), 0);
    }

    #[test]
    fn test_simple_date_3() {
        let res = date(&b"2017-01-01T01:02:03"[..]);
        assert!(res.is_ok(), "Not done: {:?}", res);
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        println!("{:#?}", calc_res);

        assert_eq!(calc_res.get_moment().unwrap().year(), 2017);
        assert_eq!(calc_res.get_moment().unwrap().month(), 1);
        assert_eq!(calc_res.get_moment().unwrap().day(), 1);
        assert_eq!(calc_res.get_moment().unwrap().hour(), 1);
        assert_eq!(calc_res.get_moment().unwrap().minute(), 2);
        assert_eq!(calc_res.get_moment().unwrap().second(), 3);
    }

    #[test]
    fn test_expressions_to_date() {
        let res = amount_expr(&b"5min + 12min"[..]);
        assert!(res.is_ok());
        let (_, o) = res.unwrap();

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
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
        assert!(res.is_ok());
        let (_, o) = res.unwrap();

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
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
        assert!(res.is_ok(), "Not done: {:?}", res.unwrap_err());
        let (_, o) = res.unwrap();

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        // because this test is basically dependent on the current time, which is a baaaad use of
        // state in a test, we rely on `test_expressions_to_date_4()` here and assume that the
        // upper assertions are enough.
    }

    #[test]
    fn test_expressions_to_date_4() {
        let res = date(&b"2017-01-01 + 5min + 12min"[..]);
        assert!(res.is_ok(), "Not done: {:?}", res.unwrap_err());
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        let calc_res = calc_res.unwrap();
        println!("{:#?}", calc_res);

        assert_eq!(calc_res.get_moment().unwrap().year(), 2017);
        assert_eq!(calc_res.get_moment().unwrap().month(), 1);
        assert_eq!(calc_res.get_moment().unwrap().day(), 1);
        assert_eq!(calc_res.get_moment().unwrap().hour(), 0);
        assert_eq!(calc_res.get_moment().unwrap().minute(), 17);
        assert_eq!(calc_res.get_moment().unwrap().second(), 0);
    }

    #[test]
    fn test_expressions_to_timetype() {
        let res = timetype(&b"5min + 12min + 15hours"[..]);
        assert!(res.is_ok());
        let (_, o) = res.unwrap();

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
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
        assert!(res.is_ok(), "Not done: {:?}", res.unwrap_err());
        let (_, o) = res.unwrap();

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
        let calc_res = calc_res.calculate();
        assert!(calc_res.is_ok());

        // because this test is basically dependent on the current time, which is a baaaad use of
        // state in a test, we rely on `test_expressions_to_date_4()` here and assume that the
        // upper assertions are enough.
    }

    #[test]
    fn test_expressions_to_timetype_subtract() {
        let res = timetype(&b"5min + 12min + 15hours - 1hour"[..]);
        assert!(res.is_ok());
        let (_, o) = res.unwrap();

        println!("{:#?}", o);

        let calc_res: crate::timetype::TimeType = o.into_timetype().unwrap();
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
