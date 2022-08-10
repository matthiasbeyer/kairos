use timetype::TimeType;

use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum Error {
    #[error("Unknown Error")]
    UnknownError,

    #[error("Cannot add: {0:?} + {1:?}")]
    CannotAdd(TimeType, TimeType),

    #[error("Cannot subtract: {0:?} - {1:?}")]
    CannotSub(TimeType, TimeType),

    #[error("The passed argument is not an amount: {0:?}")]
    ArgumentErrorNotAnAmount(TimeType),

    #[error("The passed argument is not a moment, but a {0}")]
    ArgumentErrorNotAMoment(&'static str),

    #[error("Argument Error: Cannot calculate end-of-year on a {0:?}")]
    CannotCalculateEndOfYearOn(TimeType),

    #[error("Argument Error: Cannot calculate end-of-month on a {0:?}")]
    CannotCalculateEndOfMonthOn(TimeType),

    #[error("Cannot compare Day to non-Moment TimeType: {0:?}")]
    CannotCompareDayTo(&'static str),

    #[error("Cannot compare Month to non-Moment TimeType: {0:?}")]
    CannotCompareMonthTo(&'static str),

    #[error("Out of bounds: {0}-{1}-{2}T{3}:{4}:{5}")]
    OutOfBounds(i32, u32, u32, u32, u32, u32),

    #[error("Cannot calculate date for iterator")]
    NotADateInsideIterator,

    #[error("Unknown parser error")]
    UnknownParserError,

    #[error("Tokenizer error")]
    NomError(#[from] nom::Err),
}
