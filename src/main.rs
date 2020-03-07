extern crate pest;
#[macro_use]
extern crate pest_derive;

mod briqs_parser;
use briqs_parser::parse_briqs;

fn main() {
    parse_briqs();
}
