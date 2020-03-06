extern crate pest;
#[macro_use]
extern crate pest_derive;

mod json_parser;
use json_parser::parse_json;

fn main() {
    parse_json();
}
