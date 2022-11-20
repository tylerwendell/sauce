
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct LISPParser;

fn main() {
    let successful_parse = LISPParser::parse(Rule::SAUCY, "+ 3 4");
    println!("{:?}", successful_parse);

    let unsuccessful_parse = LISPParser::parse(Rule::SAUCY, "this is not lisp");
    println!("{:?}", unsuccessful_parse);
}