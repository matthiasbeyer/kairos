use nom::whitespace::sp;

use error::Result;
use error::Error;
use parser::timetype::*;
use timetype::IntoTimeType;
use timetype;
use iter;

named!(pub iter_spec<Iterspec>, alt_complete!(
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

named!(pub until_spec<UntilSpec>, alt_complete!(
    do_parse!(
        tag!("until") >> sp >>
        exact: exact_date_parser >>
        (UntilSpec::Exact(exact))
    ) |
    do_parse!(
        num: integer >> sp >>
        tag!("times") >>
        (UntilSpec::Times(num))
    )
));

#[derive(Debug, PartialEq, Eq)]
pub enum UntilSpec {
    Exact(ExactDate),
    Times(i64)
}

named!(pub iterator<Iterator>, do_parse!(
    opt!(sp) >> d: date         >>
    opt!(sp) >> spec: iter_spec >>
    opt!(sp) >> until: opt!(complete!(until_spec)) >>
    (Iterator(d, spec, until))
));

#[derive(Debug, PartialEq, Eq)]
pub struct Iterator(Date, Iterspec, Option<UntilSpec>);

impl Iterator {
    pub fn into_user_iterator(self) -> Result<UserIterator<iter::Iter>> {
        use iter::Times;
        use iter::Until;

        let unit_to_amount = |i, unit| match unit {
            Unit::Second => timetype::TimeType::seconds(i),
            Unit::Minute => timetype::TimeType::minutes(i),
            Unit::Hour   => timetype::TimeType::hours(i),
            Unit::Day    => timetype::TimeType::days(i),
            Unit::Week   => timetype::TimeType::weeks(i),
            Unit::Month  => timetype::TimeType::months(i),
            Unit::Year   => timetype::TimeType::years(i),
        };

        let recur = match self.1 {
            Iterspec::Every(i, unit) => unit_to_amount(i, unit),
            Iterspec::Secondly => unit_to_amount(1, Unit::Second),
            Iterspec::Minutely => unit_to_amount(1, Unit::Minute),
            Iterspec::Hourly   => unit_to_amount(1, Unit::Hour),
            Iterspec::Daily    => unit_to_amount(1, Unit::Day),
            Iterspec::Weekly   => unit_to_amount(1, Unit::Week),
            Iterspec::Monthly  => unit_to_amount(1, Unit::Month),
            Iterspec::Yearly   => unit_to_amount(1, Unit::Year),
        };

        let into_ndt = |e: timetype::TimeType| e.calculate()?
            .get_moment()
            .ok_or(Error::NotADateInsideIterator)
            .map_err(Error::from)
            .map(Clone::clone);

        match self.2 {
            Some(UntilSpec::Exact(e)) => {
                let base = into_ndt(self.0.into_timetype()?)?;
                let e    = into_ndt(e.into_timetype()?)?;

                iter::Iter::build(base, recur)
                    .map(|it| UserIterator::UntilIterator(it.until(e)))
            },

            Some(UntilSpec::Times(i)) => {
                let base = into_ndt(self.0.into_timetype()?)?;
                iter::Iter::build(base, recur)
                    .map(|it| it.times(i))
                    .map(UserIterator::TimesIter)
            },

            None => {
                let base = into_ndt(self.0.into_timetype()?)?;
                iter::Iter::build(base, recur)
                    .map(UserIterator::Iterator)
            },
        }
    }
}

// names are hard
#[derive(Debug)]
pub enum UserIterator<I>
    where I: ::std::iter::Iterator<Item = Result<timetype::TimeType>>
{
    Iterator(iter::Iter),
    TimesIter(iter::TimesIter<I>),
    UntilIterator(iter::UntilIter<I>)
}

impl<I> ::std::iter::Iterator for UserIterator<I>
    where I: ::std::iter::Iterator<Item = Result<timetype::TimeType>>
{
    type Item = Result<timetype::TimeType>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            UserIterator::Iterator(ref mut i)      => i.next(),
            UserIterator::TimesIter(ref mut i)     => i.next(),
            UserIterator::UntilIterator(ref mut i) => i.next(),
        }
    }
}


#[cfg(test)]
mod tests {
    use nom::IResult;
    use super::*;

    use chrono::Timelike;
    use chrono::Datelike;

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
    fn test_iterator_1() {
        let res = iterator(&b"2017-01-01 hourly"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for hour in 0..10 { // 10 is randomly chosen (fair dice roll... )
            let n = ui.next().unwrap();
            assert!(n.is_ok(), "Not ok: {:#?}", n);
            let tt = n.unwrap();
            assert_eq!(tt.get_moment().unwrap().year()  , 2017);
            assert_eq!(tt.get_moment().unwrap().month() , 01);
            assert_eq!(tt.get_moment().unwrap().day()   , 01);
            assert_eq!(tt.get_moment().unwrap().hour()  , hour);
            assert_eq!(tt.get_moment().unwrap().minute(), 00);
            assert_eq!(tt.get_moment().unwrap().second(), 00);
        }
    }

    #[test]
    fn test_iterator_2() {
        let res = iterator(&b"2017-01-01 every 2mins"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for min in (0..60).into_iter().filter(|n| n % 2 == 0) {
            let n = ui.next().unwrap();
            assert!(n.is_ok(), "Not ok: {:#?}", n);
            let tt = n.unwrap();
            assert_eq!(tt.get_moment().unwrap().year()  , 2017);
            assert_eq!(tt.get_moment().unwrap().month() , 01);
            assert_eq!(tt.get_moment().unwrap().day()   , 01);
            assert_eq!(tt.get_moment().unwrap().hour()  , 00);
            assert_eq!(tt.get_moment().unwrap().minute(), min);
            assert_eq!(tt.get_moment().unwrap().second(), 00);
        }
    }

    #[test]
    fn test_iterator_3() {
        let res = iterator(&b"2017-01-01 daily"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for day in 1..30 {
            let n = ui.next().unwrap();
            assert!(n.is_ok(), "Not ok: {:#?}", n);
            let tt = n.unwrap();
            assert_eq!(tt.get_moment().unwrap().year()  , 2017);
            assert_eq!(tt.get_moment().unwrap().month() , 01);
            assert_eq!(tt.get_moment().unwrap().day()   , day);
            assert_eq!(tt.get_moment().unwrap().hour()  , 00);
            assert_eq!(tt.get_moment().unwrap().minute(), 00);
            assert_eq!(tt.get_moment().unwrap().second(), 00);
        }
    }

    #[test]
    fn test_iterator_4() {
        let res = iterator(&b"2017-01-01 weekly"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for week in 0..3 {
            let n = ui.next().unwrap();
            assert!(n.is_ok(), "Not ok: {:#?}", n);
            let tt = n.unwrap();
            assert_eq!(tt.get_moment().unwrap().year()  , 2017);
            assert_eq!(tt.get_moment().unwrap().month() , 01);
            assert_eq!(tt.get_moment().unwrap().day()   , 01 + (week * 7));
            assert_eq!(tt.get_moment().unwrap().hour()  , 00);
            assert_eq!(tt.get_moment().unwrap().minute(), 00);
            assert_eq!(tt.get_moment().unwrap().second(), 00);
        }
    }

    #[test]
    fn test_until_spec_1() {
        let res = until_spec(&b"until 2017-01-01T05:00:00"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);
    }

    #[test]
    fn test_until_iterator_1() {
        let res = iterator(&b"2017-01-01 hourly until 2017-01-01T05:00:00"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();
        println!("Okay: {:#?}", ui);

        for hour in 0..10 { // 10 is randomly chosen (fair dice roll... )
            if hour > 4 {
                let n = ui.next();
                assert!(n.is_none(), "Is Some, should be None: {:?}", n);
                return;
            } else {
                let n = ui.next().unwrap();
                assert!(n.is_ok(), "Not ok: {:#?}", n);
                let tt = n.unwrap();
                assert_eq!(tt.get_moment().unwrap().year()  , 2017);
                assert_eq!(tt.get_moment().unwrap().month() , 01);
                assert_eq!(tt.get_moment().unwrap().day()   , 01);
                assert_eq!(tt.get_moment().unwrap().hour()  , hour);
                assert_eq!(tt.get_moment().unwrap().minute(), 00);
                assert_eq!(tt.get_moment().unwrap().second(), 00);
            }
        }
    }

    #[test]
    fn test_until_iterator_2() {
        let res = iterator(&b"2017-01-01 every 2mins until 2017-01-01T00:10:00"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for min in (0..60).into_iter().filter(|n| n % 2 == 0) {
            if min > 9 {
                let n = ui.next();
                assert!(n.is_none(), "Is Some, should be None: {:?}", n);
                return;
            } else {
                let n = ui.next().unwrap();
                assert!(n.is_ok(), "Not ok: {:#?}", n);
                let tt = n.unwrap();
                assert_eq!(tt.get_moment().unwrap().year()  , 2017);
                assert_eq!(tt.get_moment().unwrap().month() , 01);
                assert_eq!(tt.get_moment().unwrap().day()   , 01);
                assert_eq!(tt.get_moment().unwrap().hour()  , 00);
                assert_eq!(tt.get_moment().unwrap().minute(), min);
                assert_eq!(tt.get_moment().unwrap().second(), 00);
            }
        }
    }

    #[test]
    fn test_until_iterator_3() {
        let res = iterator(&b"2017-01-01 daily until 2017-01-05"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for day in 1..30 {
            if day > 4 {
                let n = ui.next();
                assert!(n.is_none(), "Is Some, should be None: {:?}", n);
                return;
            } else {
                let n = ui.next().unwrap();
                assert!(n.is_ok(), "Not ok: {:#?}", n);
                let tt = n.unwrap();
                assert_eq!(tt.get_moment().unwrap().year()  , 2017);
                assert_eq!(tt.get_moment().unwrap().month() , 01);
                assert_eq!(tt.get_moment().unwrap().day()   , day);
                assert_eq!(tt.get_moment().unwrap().hour()  , 00);
                assert_eq!(tt.get_moment().unwrap().minute(), 00);
                assert_eq!(tt.get_moment().unwrap().second(), 00);
            }
        }
    }

    #[test]
    fn test_until_iterator_4() {
        let res = iterator(&b"2017-01-01 weekly until 2017-01-14"[..]);
        assert!(res.is_done(), format!("Not done: {:?}", res));
        let (_, i) = res.unwrap();
        println!("{:#?}", i);

        let ui : Result<UserIterator<iter::Iter>> = i.into_user_iterator();
        assert!(ui.is_ok(), "Not okay: {:#?}", ui);
        let mut ui = ui.unwrap();

        for week in 0..3 {
            if (week * 7) > 13 {
                let n = ui.next();
                assert!(n.is_none(), "Is Some, should be None: {:?}", n);
                return;
            } else {
                let n = ui.next().unwrap();
                assert!(n.is_ok(), "Not ok: {:#?}", n);
                let tt = n.unwrap();
                assert_eq!(tt.get_moment().unwrap().year()  , 2017);
                assert_eq!(tt.get_moment().unwrap().month() , 01);
                assert_eq!(tt.get_moment().unwrap().day()   , 01 + (week * 7));
                assert_eq!(tt.get_moment().unwrap().hour()  , 00);
                assert_eq!(tt.get_moment().unwrap().minute(), 00);
                assert_eq!(tt.get_moment().unwrap().second(), 00);
            }
        }
    }
}

