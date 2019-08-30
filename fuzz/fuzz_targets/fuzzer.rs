#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate dottools;

use dottools::{clean_string, Graph};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let g = s.parse::<Graph>().expect("parse returns a graph");
        assert_eq!(clean_string(s).trim_end(), g.to_string());
    }
});
