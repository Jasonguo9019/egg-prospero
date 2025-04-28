use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;


mod prospero;



fn main() {
    let input = "/Users/jasonguo/egg-prospero/prospero.vm";
    let start = prospero::parse_lang_expr(input).unwrap();
    //let start: RecExpr<prospero::Prospero> = "(max (neg (+ 10 5)) 1)".parse().unwrap();
    //println!("Starting expression: {}", start);

    let rules: &[Rewrite<prospero::Prospero, prospero::ConstantFold>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        ];
    let runner: Runner<prospero::Prospero, prospero::ConstantFold> = 
        Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("{}", best_expr);
}


