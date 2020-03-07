use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "briqs.pest"]
struct BriqsParser;

enum Briq {
    Piq,
}

pub fn parse_briqs() {
    let unparsed_file = fs::read_to_string("_.iq").expect("cannot read file");

    let file = BriqsParser::parse(Rule::briqs, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails;

    for pair in file.into_inner() {
        println!("{:?}", pair);
    }
}