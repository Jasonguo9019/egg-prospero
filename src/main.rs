use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;


mod prospero;



fn main() {
    let input = "/Users/jasonguo/egg-prospero/prospero.vm";
    let start = prospero::parse_lang_expr(input).unwrap();
    //println!("Final expression: {}", start);

    let rules: &[Rewrite<prospero::Prospero, ()>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        ];
    let runner = Runner::default().with_expr(&start).run(prospero::rules());
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
}


