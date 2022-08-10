extern crate kairos;

use kairos::timetype::TimeType as TT;

fn pretty_print(tt: TT) {
    match tt {
        TT::Seconds(e) => println!("{} Seconds", e),
        TT::Minutes(e) => println!("{} Minutes", e),
        TT::Hours(e) => println!("{} Hours", e),
        TT::Days(e) => println!("{} Days", e),
        TT::Months(e) => println!("{} Months", e),
        TT::Years(e) => println!("{} Years", e),
        TT::Moment(ndt) => println!("{} ", ndt),
        other => println!("Cannot pretty-print: '{:?}'", other),
    }
}

fn main() {
    // not sure whether this is actually fast or something, but we don't care here, do we?
    let s = ::std::env::args()
        .skip(1)
        .fold(String::new(), |acc, obj| format!("{} {}", acc, obj));
    let s = s.trim(); // because kairos is not yet whitespace tolerant

    match kairos::parser::parse(s) {
        Err(e) => println!("Error -> {:?}", e),
        Ok(kairos::parser::Parsed::TimeType(tt)) => match tt.calculate() {
            Ok(r) => pretty_print(r),
            Err(e) => println!("Error calculating: {:?}", e),
        },
        Ok(kairos::parser::Parsed::Iterator(Ok(ui))) => {
            for elem in ui {
                match elem {
                    Ok(r) => pretty_print(r),
                    Err(e) => {
                        println!("Error calculating: {:?}", e);
                        ::std::process::exit(1)
                    }
                }
            }
        }
        Ok(kairos::parser::Parsed::Iterator(Err(e))) => {
            println!("Failed building iterator: {:?}", e);
        }
    }
}
