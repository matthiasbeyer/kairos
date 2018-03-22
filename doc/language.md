# The kairos language

Kairos features a parser frontend which can be used to parse user input into
either a `TimeType` or an `UserIterator`.

This is the documentation for the language this parser understands.


## Basics

There is only addition and subtraction

```
operator = "+" | "-"
```

Numbers are valid, too.

```
digit  = "0" | "1" "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
number = digit+
```


## Units

A unit specifies the Unit of a number

```
second_unit = "seconds" | "second" | "secs" | "sec" | "s"
minute_unit = "minutes" | "minute" | "mins" | "min"
hour_unit   = "hours" | "hour" | "hrs" | "hr"
day_unit    = "days" | "day" | "d"
week_unit   = "weeks" | "week" | "w"
month_unit  = "months" | "month"
year_unit   = "years" | "year" | "yrs"

unit =
  second_unit |
  minute_unit |
  hour_unit   |
  day_unit    |
  week_unit   |
  month_unit  |
  year_unit   |
```


## Aliases

An alias can be used for specifying "one of a <unit>". For example "1second"
can be specified as "secondly"

```
second_alias = "secondly"
minute_alias = "minutely"
hour_alias   = "hourly"
day_alias    = "daily"
week_alias   = "weekly"
month_alias  = "monthly"
year_alias   = "yearly"
```


## Amounts

An amount is a number plus a unit, or an alias

```
second_amount = number second_unit | second_alias
minute_amount = number minute_unit | minute_alias
hour_amount   = number hour_unit   | hour_alias
day_amount    = number day_unit    | day_alias
week_amount   = number week_unit   | week_alias
month_amount  = number month_unit  | month_alias
year_amount   = number year_unit   | year_alias

amount =
  second_amount |
  minute_amount |
  hour_amount   |
  day_amount    |
  week_amount   |
  month_amount  |
  year_amount
```

## Exact dates

Exact dates can be specified in ISO 8601 format. The time is optional.
Aliases for today, yesterday and tomorrow exist.

```
two_digits = digit digit

offset = "+" two_digits two_digits
time   = two_digits (":" two_digits (":" two_digits offset?)?)?

exact_date =
  two_digits two_digits ("-" two_digits ("-" two_digits ("T" time)?)?)? |
  "today"                 |
  "yesterday"             |
  "tomorrow"              |
```

As you see, specifying only a year, a year and a month, a date, a date with an
hour, an date with an hour and a minute or a complete year-to-second is
possible as well as specifying an offset ("+0200" for example).


## Expressions

Expressions are calculations of time. There are different _kinds_ of
expressions: Simple adding and subtracting of time amounts,

```
amount_expression     = amount (operator amount_expression)?
exact_date_expression = exact_date (operator amount_expression)?
```

## TimeType

A TimeType is either a date or an amount expression:

```
timetype = exact_date_expression | amount_expression
```

That means that a TimeType either represents a specific DateTime or an amount
of time (like "Two weeks" for example).


## Specifying iterators

An iterator consists of three parts which are explained in the following.


### Until

An iterator may have an "until" specification. This tells the iterator when to
stop the iteration.

```
until_spec = "until" exact_date | number "times"
```

**Warning:** The "exact_date" in this variant must be specified from year to
day. Specifying only the year does _not_ work as expected.
This is a known bug.


### Iter specification

The "iter specification" tells the iterator in what steps to iterate.
Aliases for one of a certain unit exists:

```
iter_spec =
  "secondly" |
  "minutely" |
  "hourly"   |
  "daily"    |
  "weekly"   |
  "monthly"  |
  "yearly"   |
  number unit
```

### Iterator

The iterator itself is rather simple now:

```
iterator = date iter_spec until_spec?
```

If the `until_spec` is not given, the iterator yields new TimeType objects
**until it panics**.


