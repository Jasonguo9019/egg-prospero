use egg::{rewrite as rw, *};
use ordered_float::NotNan;
pub type Constant = NotNan<f64>;


mod prospero;



fn main() {
    let input = "/Users/jasonguo/egg-prospero/prospero.vm";
    let expr = prospero::parse_lang_expr(input).unwrap();
    println!("Final expression: {}", expr);
}


