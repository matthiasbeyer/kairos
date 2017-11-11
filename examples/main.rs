extern crate kairos;

fn main() {
    // not sure whether this is actually fast or something, but we don't care here, do we?
    let s = ::std::env::args().skip(1).fold(String::new(), |acc, obj| format!("{} {}", acc, obj));
    let s = s.trim(); // because kairos is not yet whitespace tolerant

    println!("Parsing: '{}'", s);
    match kairos::timetype::TimeType::parse(&s) {
        Ok(tt) => {
            println!("{:?}", tt);

            match tt.calculate() {
                Ok(r) => println!("{:?}", r),
                Err(e) => println!("Error calculating: {:?}", e),
            }
        },
        Err(e) => println!("Error -> {:?}", e),
    }
}
