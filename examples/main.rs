extern crate kairos;

fn main() {
    // not sure whether this is actually fast or something, but we don't care here, do we?
    let s = ::std::env::args().skip(1).fold(String::new(), |acc, obj| format!("{} {}", acc, obj));
    let s = s.trim(); // because kairos is not yet whitespace tolerant

    println!("Parsing: '{}'", s);
    match kairos::parser::parse(s) {
        Err(e) => println!("Error -> {:?}", e),
        Ok(kairos::parser::Parsed::TimeType(tt)) => {
            println!("Having TimeType");

            match tt.calculate() {
                Ok(r)  => println!("{:?}", r),
                Err(e) => println!("Error calculating: {:?}", e),
            }
        },
        Ok(kairos::parser::Parsed::Iterator(Ok(ui))) => {
            println!("Having iterator");

            for elem in ui {
                match elem {
                    Ok(r)  => println!("{:?}", r),
                    Err(e) => {
                        println!("Error calculating: {:?}", e);
                        ::std::process::exit(1)
                    }
                }
            }
        },
        Ok(kairos::parser::Parsed::Iterator(Err(e))) => {
            println!("Failed building iterator: {:?}", e);
        },
    }
}
