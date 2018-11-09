use timetype::TimeType;

#[derive(Debug, Clone, Eq, PartialEq, Fail)]
pub enum ErrorKind {

    #[fail(display = "Unknown Error")]
    UnknownError,

    #[fail(display = "Cannot add: {:?} + {:?}", _0, _1)]
    CannotAdd(TimeType, TimeType),

    #[fail(display = "Cannot subtract: {:?} - {:?}", _0, _1)]
    CannotSub(TimeType, TimeType),

    #[fail(display = "The passed argument is not an amount: {:?}", _0)]
    ArgumentErrorNotAnAmount(TimeType),

    #[fail(display = "The passed argument is not a moment, but a {}", _0)]
    ArgumentErrorNotAMoment(&'static str),

    #[fail(display = "Argument Error: Cannot calculate end-of-year on a {:?}", _0)]
    CannotCalculateEndOfYearOn(TimeType),

    #[fail(display = "Argument Error: Cannot calculate end-of-month on a {:?}", _0)]
    CannotCalculateEndOfMonthOn(TimeType),

    #[fail(display = "Cannot compare Day to non-Moment TimeType: {:?}", _0)]
    CannotCompareDayTo(&'static str),

    #[fail(display = "Cannot compare Month to non-Moment TimeType: {:?}", _0)]
    CannotCompareMonthTo(&'static str),

    #[fail(display = "Out of bounds: {}-{}-{}T{}:{}:{}", _0, _1, _2, _3, _4, _5)]
    OutOfBounds(i32, u32, u32, u32, u32, u32),

    #[fail(display = "Cannot calculate date for iterator")]
    NotADateInsideIterator,

    #[fail(display = "Unknown parser error")]
    UnknownParserError,

}
