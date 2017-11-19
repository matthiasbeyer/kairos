# Kairos

Calculate times with chrono "plain text like" in Rust.

[![Build Status](https://travis-ci.org/matthiasbeyer/kairos.svg?branch=master)](https://travis-ci.org/matthiasbeyer/kairos)
[![license](https://img.shields.io/github/license/matthiasbeyer/kairos.svg?maxAge=2592000?style=flat-square)]()
[![Tokei](https://tokei.rs/b1/github/matthiasbeyer/kairos)](https://github.com/matthiasbeyer/kairos)

From Wikipedia:

> Kairos (καιρός) is an Ancient Greek word meaning the right, critical or
> opportune moment.

This library offers an abstraction over the awesome `chrono` crate to
calculate dates almost like one would write plain text:

```rust
use kairos::timetype::TimeType as TT;

// get the end of the month of the day 5 days ago
let _ = (TT::today() - TT::weeks(1) + TT::days(2)).end_of_month();

// alternative to above
let _ = (TT::today() - TT::days(5)).end_of_month();

// NOTE: The following features are not yet included

// get a vector of dates for the next 4 weeks, starting today
let _ = TT::today()
    .every(TT::week(1))
    .take(4);

// get an iterator of dates for the next year, in a weekly fashion, starting
// today but skipping october
let _ = TT::today()
    .every(TT::week(1))
    .skip(Month::October)
    .until(Mark::END_OF_YEAR);

// and finally, a complex one

let _ = (TT::today() - TT::years(1))       // exactly one year ago
  .every(Day::Monday)                      // and then every Monday
  .skip(Month::October)                    // but not in october
  .skip(|date| date.is(Mark::MONTH_START)) // and not if the day is the 1st of a month
  .until(Mark::Moment(today()));           // until today
```

## Parsing User-Input

kairos ships a user input parser, so you can include kairos in your
commandline applications.

Have a look at [the example CLI utility](./examples/main.rs).

# License

MPL 2.0

