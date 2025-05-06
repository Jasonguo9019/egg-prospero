mod prospero;
mod interval;
use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;





fn main() {
    let input = "/Users/jasonguo/egg-prospero/prospero.vm";
    //let start = prospero::parse_lang_expr(input).unwrap();
    let start: RecExpr<prospero::Prospero> = "(+ (intv 10 20) (intv 10 30))".parse().unwrap();
    //println!("Starting expression: {}", start);

    let rules: &[Rewrite<prospero::Prospero, prospero::IntervalArithmetic>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        ];
    let runner: Runner<prospero::Prospero, prospero::IntervalArithmetic> = 
        Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("{}", best_expr);
}


