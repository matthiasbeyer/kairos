use timetype::TimeType;

error_chain! {
    types {
        KairosError, KairosErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        NomError(::nom::simple_errors::Err);
    }

    errors {

        UnknownError {
            description("Unknown Error")
            display("Unknown Error")
        }

        CannotAdd(a: TimeType, b: TimeType) {
            description("Cannot add")
            display("Cannot add: {:?} + {:?}", a, b)
        }

        CannotSub(a: TimeType, b: TimeType) {
            description("Cannot subtract")
            display("Cannot subtract: {:?} - {:?}", a, b)
        }

        ArgumentErrorNotAnAmount(tt: TimeType) {
            description("Argument Error: Not an amount TimeType object")
            display("The passed argument is not an amount: {:?}", tt)
        }

        ArgumentErrorNotAMoment(name: &'static str) {
            description("Argument Error: Not a moment TimeType object")
            display("The passed argument is not a moment, but a {}", name)
        }

        CannotCalculateEndOfYearOn(tt: TimeType) {
            description("Argument Error: Cannot calculate end-of-year")
            display("Argument Error: Cannot calculate end-of-year on a {:?}", tt)
        }

        CannotCalculateEndOfMonthOn(tt: TimeType) {
            description("Argument Error: Cannot calculate end-of-month")
            display("Argument Error: Cannot calculate end-of-month on a {:?}", tt)
        }

        CannotCompareDayTo(tt_rep: &'static str) {
            description("Cannot compare Day to non-Moment TimeType")
            display("Cannot compare Day to non-Moment TimeType: {:?}", tt_rep)
        }

        CannotCompareMonthTo(tt_rep: &'static str) {
            description("Cannot compare Month to non-Moment TimeType")
            display("Cannot compare Month to non-Moment TimeType: {:?}", tt_rep)
        }

        NotADateInsideIterator {
            description("Cannot calculate date for iterator")
            display("Cannot calculate date for iterator")
        }

        UnknownParserError {
            description("Unknown parser error")
            display("Unknown parser error")
        }

    }

}
