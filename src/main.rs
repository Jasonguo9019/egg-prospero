mod prospero;
mod interval;
use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;





fn main() {

    let input  = "/Users/jasonguo/egg-prospero/prospero.vm";
    let start  = prospero::parse_lang_expr(input).unwrap();
    //let start: RecExpr<prospero::Prospero> = "(* (+ 2 5) var-x)".parse().unwrap();
    
    let mut runner_plain: Runner<prospero::Prospero, ()> = Runner::default()
        .with_expr(&start);
    let plain_ex = Extractor::new(&runner_plain.egraph, AstSize);
    let (plain_cost, plain_expr) = plain_ex.find_best(runner_plain.roots[0]);
    println!("Before: cost={} expr={}", plain_cost, plain_expr);

    let mut runner_ia: Runner<prospero::Prospero, prospero::IntervalArithmetic> = Runner::default()
        .with_expr(&start);
    runner_ia = runner_ia.run(&[]);
    let ia_ex = Extractor::new(&runner_ia.egraph, AstSize);
    let (ia_cost, ia_expr) = ia_ex.find_best(runner_ia.roots[0]);
    println!("After (with interval arithmetic):   cost={} expr={}", ia_cost, ia_expr);
}


    


