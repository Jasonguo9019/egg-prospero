use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;


define_language! {
    pub enum Prospero {
        "var-x" = VarX,
        "var-y" = VarY,
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "max" = Test([Id; 2]),
        "min" = Min([Id; 2]),
        "max" = Max([Id; 2]),
        "neg" = Neg(Id),
        "square" = Square(Id),
        "sqrt" = Sqrt(Id),
        Constant(Constant),
    }
}


pub fn parse_lang_expr(input: &str) -> io::Result<RecExpr<Prospero>> {
    let mut Ids: HashMap<String, Id> = HashMap::new();
    let mut list = Vec::new();
    println!("{}", input);
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() { continue; }

        let id = tokens[0];
        let node = match tokens[1] {
            "const" => Prospero::Constant(tokens[2].parse().unwrap()),
            "var-x" => Prospero::VarX,
            "var-y" => Prospero::VarY,
            "neg"   => Prospero::Neg(Ids[tokens[2]]),
            "square" => Prospero::Square(Ids[tokens[2]]), 
            "sqrt" => Prospero::Sqrt(Ids[tokens[2]]),
            "add" => Prospero::Add([Ids[tokens[2]], Ids[tokens[3]]]),
            "sub" => Prospero::Sub([Ids[tokens[2]], Ids[tokens[3]]]),
            "mul" => Prospero::Mul([Ids[tokens[2]], Ids[tokens[3]]]),
            "max" => Prospero::Max([Ids[tokens[2]], Ids[tokens[3]]]),
            "min" => Prospero::Min([Ids[tokens[2]], Ids[tokens[3]]]),
            _ => panic!("Unknown operation: {}", tokens[1]),
        };

        let index = list.len();
        list.push(node);
        Ids.insert(id.to_string(), index.into());
    }
    println!("{:?}", list);
    return Ok(egg::RecExpr::<Prospero>::from(list));
}

pub fn test() {
    println!("Hello, world!");
}