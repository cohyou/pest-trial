extern crate pest; 
#[macro_use]
extern crate pest_derive;


use pest::Parser;


#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

fn main() {
    try_ident_parse();
}

fn try_ident_parse() {
    let pairs_pos_int = IdentParser::parse(Rule::pos_int, "99").unwrap_or_else(|e| panic!("{}", e));
    println!("pairs_pos_int = {}", pairs_pos_int.as_str());

    let pairs_digit_int = IdentParser::parse(Rule::digit_int, "0123").unwrap_or_else(|e| panic!("{}", e));
    println!("pairs_digit_int = {}", pairs_digit_int.as_str());

    let pairs_digit_int = IdentParser::parse(Rule::digit_int, "0123456").unwrap_or_else(|e| panic!("{}", e));
    println!("pairs_digit_int = {}", pairs_digit_int.as_str());
}
