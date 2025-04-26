use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub type Rewrite = egg::Rewrite<Prospero, ()>;


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

/*
#[rustfmt::skip]
pub fn rules() -> Vec<Rewrite> { vec![
    rw!("comm-add";  "(+ ?a ?b)"  => "(+ ?b ?a)"),
]}
*/

pub fn parse_lang_expr(input: &str) -> io::Result<RecExpr<Prospero>> {
    let mut ids: HashMap<String, Id> = HashMap::new();
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
            "neg"   => Prospero::Neg(ids[tokens[2]]),
            "square" => Prospero::Square(ids[tokens[2]]), 
            "sqrt" => Prospero::Sqrt(ids[tokens[2]]),
            "add" => Prospero::Add([ids[tokens[2]], ids[tokens[3]]]),
            "sub" => Prospero::Sub([ids[tokens[2]], ids[tokens[3]]]),
            "mul" => Prospero::Mul([ids[tokens[2]], ids[tokens[3]]]),
            "max" => Prospero::Max([ids[tokens[2]], ids[tokens[3]]]),
            "min" => Prospero::Min([ids[tokens[2]], ids[tokens[3]]]),
            _ => panic!("Unknown operation: {}", tokens[1]),
        };

        let index = list.len();
        list.push(node);
        ids.insert(id.to_string(), index.into());
    }
    println!("{:?}", list);
    return Ok(egg::RecExpr::<Prospero>::from(list));
}

