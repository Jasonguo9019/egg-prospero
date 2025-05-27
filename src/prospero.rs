use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::interval::Interval;
pub type EGraph = egg::EGraph<Prospero, IntervalArithmetic>;
pub type Rewrite = egg::Rewrite<Prospero, ()>;


define_language! {
    pub enum Prospero {
        "var-x" = VarX,
        "var-y" = VarY,
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "min" = Min([Id; 2]),
        "max" = Max([Id; 2]),
        "neg" = Neg(Id),
        "square" = Square(Id),
        "sqrt" = Sqrt(Id),
        Constant(Constant),
        "intv" = Interval([Id; 2]),
    }
}
#[derive(Default)]
pub struct IntervalArithmetic;
impl Analysis<Prospero> for IntervalArithmetic {
    type Data = Option<(Interval, PatternAst<Prospero>)>;

    fn make(egraph: &mut EGraph, enode: &Prospero) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| &d.0);

        Some(match enode {
            /*
            Prospero::VarX | Prospero::VarY => (
                Interval { lo: -1.0, hi: 1.0 },
                format!("{}", enode).parse().unwrap(),
            ),    
            */
            Prospero::Constant(c) => (
                Interval::constant(**c),
                format!("{}", c).parse().unwrap(),
            ),
            
            Prospero::Interval([a, b]) => (
                Interval {lo: x(a)?.lo, hi: x(b)?.hi},
                format!("(intv {} {})", x(a)?, x(b)?).parse().unwrap(), 
            ),
            Prospero::Add([a, b]) => (
                Interval::add(x(a)?, x(b)?), 
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Sub([a, b]) => (
                Interval::sub(x(a)?, x(b)?),
                format!("(- {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Mul([a, b]) => (
                Interval::mul(x(a)?, x(b)?),
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Neg(a) => (
                Interval::neg(x(a)?),
                format!("(neg {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Square(a) => (
                Interval::square(x(a)?),
                format!("(square {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Sqrt(a) => (
                Interval::sqrt(x(a)?),
                format!("(sqrt {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Min([a, b]) => (
                Interval::min(x(a)?, x(b)?),
                format!("(min {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Max([a, b]) => (
                Interval::max(x(a)?, x(b)?),
                format!("(max {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            let Interval { lo: lo1, hi: hi1 } = a.0;
            let Interval { lo: lo2, hi: hi2 } = b.0;
            let merged = Interval {
                lo: lo1.min(lo2),
                hi: hi1.max(hi2),
            };
            DidMerge(a.0 != merged, b.0 != merged)
        })
    }
        

    fn modify(egraph: &mut EGraph, id: Id) {
        let data = egraph[id].data.clone();
        if let Some((interval, pat)) = data {
            if egraph.are_explanations_enabled() {
                let interval_str = interval.to_string();
                dbg!(&interval_str);
                egraph.union_instantiations(
                    &pat,
                    &interval_str.parse().unwrap(),
                    &Default::default(),
                    "interval_analysis".to_string(),
                );
            } else {
                let id_lo = egraph.add(Prospero::Constant(NotNan::new(interval.lo).unwrap()));
                let id_hi = egraph.add(Prospero::Constant(NotNan::new(interval.hi).unwrap()));

                let added = egraph.add(Prospero::Interval([id_lo, id_hi]));
                egraph.union(id, added);
            }

        }
    }
}

/*
#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Prospero> for ConstantFold {
    type Data = Option<(Constant, PatternAst<Prospero>)>;

    fn make(egraph: &mut EGraph, enode: &Prospero) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0);
        Some(match enode {
            Prospero::Constant(c) => (*c, format!("{}", c).parse().unwrap()),
            Prospero::Add([a, b]) => (
                x(a)? + x(b)?,
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Sub([a, b]) => (
                x(a)? - x(b)?,
                format!("(- {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Mul([a, b]) => (
                x(a)? * x(b)?,
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Square(a) => (
                x(a)? * x(a)?,
                format!("(square {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Neg(a) => (
                -x(a)?,
                format!("(neg {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Sqrt(a) => (
                NotNan::<f64>::new(x(a)?.sqrt()).unwrap(),
                format!("(sqrt {})", x(a)?).parse().unwrap(),
            ),
            Prospero::Max([a, b]) => (
                x(a)?.max(x(b)?),
                format!("(max {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Prospero::Min([a, b]) => (
                x(a)?.min(x(b)?),
                format!("(min {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let data = egraph[id].data.clone();
        if let Some((c, pat)) = data {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &pat,
                    &format!("{}", c).parse().unwrap(),
                    &Default::default(),
                    "constant_fold".to_string(),
                );
            } else {
                let added = egraph.add(Prospero::Constant(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

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
    //println!("{:?}", list);
    return Ok(egg::RecExpr::<Prospero>::from(list));
}

