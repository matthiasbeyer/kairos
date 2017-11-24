#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate kairos;

fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = kairos::parser::parse(data);
    }
});
