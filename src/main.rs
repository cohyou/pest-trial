extern crate pest;
#[macro_use]
extern crate pest_derive;


use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
struct CSVParser;


fn main() {
    try_csv_parse();
    // try_ident_parse();
}

fn try_csv_parse() {
    let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    println!("{:?}", unsuccessful_parse);
}
