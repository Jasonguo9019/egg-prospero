mod prospero;
mod interval;
use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;





fn main() {
    
    let input = "/Users/jasonguo/egg-prospero/prospero.vm";
    //let start = prospero::parse_lang_expr(input).unwrap();
    let start: RecExpr<prospero::Prospero> = "(+ var-x var-y)".parse().unwrap();
    //println!("Starting expression: {}", start);
    let rules: &[Rewrite<prospero::Prospero, prospero::IntervalArithmetic>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        ];
    
    let mut runner = Runner::default().with_expr(&start);
    let pre_extractor = Extractor::new(&runner.egraph, AstSize);
    let (pre_cost, pre_expr) = pre_extractor.find_best(runner.roots[0]);
    println!("before:  cost={}  expr={}", pre_cost, pre_expr);

    runner = runner.run(rules);

    let post_extractor = Extractor::new(&runner.egraph, AstSize);
    let (post_cost, post_expr) = post_extractor.find_best(runner.roots[0]);
    println!("after:  cost={}  expr={}", post_cost, post_expr);
}


