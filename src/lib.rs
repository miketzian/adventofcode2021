// https://github.com/rust-lang/rust/issues/44342
#[cfg_attr(test, macro_use)]
extern crate lazy_static;

#[cfg_attr(test, macro_use)]
extern crate assert_matches;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod util;
