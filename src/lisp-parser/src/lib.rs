extern crate pest;
#[macro_use]
extern crate pest_derive;

use self::AstNode::*;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct LISPParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MonadicVerb {
    GreaterThan,
    GreaterThanEqual,
    Multiplication,
    Difference,
    Addition,
    Division,
    Tally,
    LessThan,
    LessThanEqual,
    Equivalency,
    NotEquivalent,
    Power,
    Or,
    And,
}

use operators::Primitive;
mod operators;

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Value(Primitive),
    MonadicOp {
        operator: MonadicVerb,
        expr: Box<AstNode>,
    },
    Terms(Vec<AstNode>),
    IsGlobal {
        ident: String,
        expr: Box<AstNode>,
    },
    Ident(String),
}



pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = LISPParser::parse(Rule::lisp, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                ast.push(build_ast_from_expr(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::monadicExpr => {
            let mut pair = pair.into_inner();
            let operator = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            parse_monadic_verb(operator, expr)
        }
        Rule::terms => {
            let terms: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();
            // If there's just a single term, return it without
            // wrapping it in a Terms node.
            match terms.len() {
                1 => terms.get(0).unwrap().clone(),
                _ => Terms(terms),
            }
        }
        Rule::assgmtExpr => {
            let mut pair = pair.into_inner();
            let ident = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let expr = build_ast_from_expr(expr);
            AstNode::IsGlobal {
                ident: String::from(ident.as_str()),
                expr: Box::new(expr),
            }
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}


fn parse_monadic_verb(pair: pest::iterators::Pair<Rule>, expr: AstNode) -> AstNode {
    AstNode::MonadicOp {
        operator: match pair.as_str() {
            ">" => MonadicVerb::GreaterThan,
            ">=" => MonadicVerb::GreaterThanEqual,
            "*" => MonadicVerb::Multiplication,
            "-" => MonadicVerb::Difference,
            "+" => MonadicVerb::Addition,
            "/" => MonadicVerb::Division,
            "#" => MonadicVerb::Tally,
            "<" => MonadicVerb::LessThan,
            "<=" => MonadicVerb::LessThan,
            "==" => MonadicVerb::Equivalency,
            "!=" => MonadicVerb::NotEquivalent,
            "^" => MonadicVerb::Power,
            "|" =>MonadicVerb::Or,
            "&" =>MonadicVerb::And,
            _ => panic!("Unsupported monadic operator: {}", pair.as_str()),
        },
        expr: Box::new(expr),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::value => build_ast_from_values(pair.into_inner().next().unwrap()),
        Rule::expr => build_ast_from_expr(pair),
        Rule::ident => AstNode::Ident(String::from(pair.as_str())),
        unknown_term => panic!("Unexpected term: {:?}", unknown_term),
    }
}

fn build_ast_from_values(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "_" => (-1, &istr[1..]),
                _ => (1, &istr[..]),
            };
            let integer: i32 = istr.parse().unwrap();
            Value(Primitive::Integer(sign * integer))
        }
        Rule::decimal => {
            let dstr = pair.as_str();
            let (sign, dstr) = match &dstr[..1] {
                "_" => (-1.0, &dstr[1..]),
                _ => (1.0, &dstr[..]),
            };
            let mut flt: f64 = dstr.parse().unwrap();
            if flt != 0.0 {
                // Avoid negative zeroes; only multiply sign by nonzeroes.
                flt *= sign;
            }
            Value(Primitive::DoublePrecisionFloat(flt))
        }
        Rule::string => {
            let s = &pair.as_str();
            // Strip leading and ending quotes.
            let s = &s[1..s.len() - 1];
            // Escaped string quotes become single quotes here.
            println!("this is the value: _{:?}_", pair.as_rule());
            let s = s.replace("''", "'");
            Value(Primitive::Str(s))
        }
        unknown_term => {
            panic!("Unexpected term: {:?}", unknown_term)
        },
    }
}

// fn main() {
//     // let unparsed_file = std::fs::read_to_string("+ 1 2").expect("cannot read ijs file");
//     let astnode = parse("+ 5 (* 2 2)").expect("unsuccessful parse");
//     println!("{:?}", &astnode);

//     let astnode = parse("m:=5").expect("unsuccessful parse");
//     println!("{:?}", &astnode);

//     let astnode = parse("a:='Apple'").expect("unsuccessful parse");
//     println!("{:?}", &astnode);
// }

// pub fn evaluate(ast: Vec<AstNode>) -> Result<AstNode, Error<Rule>> {
//     let peel = ast[0].clone();
//     match peel {
//         AstNode::MonadicOp { operator, expr } => {
//             // match base.operator {
//             //     GreaterThan=> ,
//             //     GreaterThanEqual=> ,
//             //     Multiplication=> ,
//             //     Difference=>,
//             //     Addition=>,
//             //     Division=>,
//             //     Tally=>,
//             //     LessThan=>,
//             //     LessThanEqual=>,
//             //     Equivalency=>,
//             //     NotEquivalent=>,
//             //     Power=>,
//             //     Or=>,
//             //     And=>,
//             // }
//             println!("1 The first thing is a function with operator: {:#?}", operator);

//             println!("1 The first thing is a function and parameters: {:?}", expr);
//             let mut expV  = vec![];
//             expV.push(*expr);
//             let ans = evaluate(expV);
//             println!("1 The first thing is assignment and value: {:?}", ans.unwrap());
//             Ok(Str("todo".to_string()))
//         },
//         AstNode::IsGlobal { ident, expr } => {
//             println!("1 The first thing is assignment with var name: {:#?}", ident);
//             let mut expV  = vec![];
//             expV.push(*expr);
//             let ans = evaluate(expV);
//             println!("1 The first thing is assignment and value: {:?}", ans.unwrap());
//             Ok(Str("todo".to_string()))
//         },
//         Integer(i) => {
//             Ok(Integer(i))
//         },
//         DoublePrecisionFloat(f) => Ok(DoublePrecisionFloat(f)),
//         Terms(t) => Ok(Terms(t)),
//         Ident(v) => Ok(Str(v)),
//         Str(s) => Ok(Str(s)),

//     }
 
// }


// fn addition<T>(ast: Vec<AstNode>) -> Result<T, Error<Rule>> {
//     let sum: Vec<T> = Vec::new();
//     for item in ast{
//         sum.push(item)
//     }
//     Ok()
// }