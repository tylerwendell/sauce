
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct LISPParser;

fn main() {
    let successful_parse = LISPParser::parse(Rule::SAUCY, "+ 3 4");
    // println!("{:?}", successful_parse);

    let lisp: LispAst = parse_json_file("+ 3 4").expect("unsuccessful parse");
    println!("{:?}", lisp)

    // let unsuccessful_parse = LISPParser::parse(Rule::SAUCY, "this is not lisp");
    // println!("{:?}", unsuccessful_parse);
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum OperatorVerb {
    Sum,
    Difference,
    Multiplication,
    Division,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LispAst {
    Number(f64),
    Operator(OperatorVerb),
    Expr(Vec<LispAst>),
    Function{
        operator: OperatorVerb,
        number: Vec<LispAst>
    },
}


fn parse_json_file(input: &str) -> Result<LispAst, Error<Rule>> {
    use pest::iterators::Pair;

    let parse = LISPParser::parse(Rule::SAUCY, input)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> LispAst {
        println!("{}", pair);
        match pair.as_rule() {
            Rule::operator => parse_operator(pair.into_inner().next().unwrap(), ),
            Rule::expr => parse_value(pair.into_inner().next().unwrap()),
            Rule::number => LispAst::Number(pair.as_str().parse().unwrap()),
            Rule::SAUCY
            | Rule::EOI
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(parse))
}

fn parse_operator(pair: pest::iterators::Pair<Rule>) -> LispAst {
    LispAst::Operator(match pair.as_str() {
            "+" => OperatorVerb::Sum,
            "*" => OperatorVerb::Multiplication,
            "-" => OperatorVerb::Difference,
            "/" => OperatorVerb::Division,
            _ => panic!("Unsupported monadic verb: {}", pair.as_str()),
       })
}