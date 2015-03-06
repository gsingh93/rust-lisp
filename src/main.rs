#![feature(plugin, core)]
#![plugin(peg_syntax_ext)]

use lisp::expression;

peg_file! lisp("grammar.rustpeg");

fn main() {
    assert_eq!(expression("1"), Ok(1));
    assert_eq!(expression("(    +   1    1   )"), Ok(2));
    assert_eq!(expression("(+ 1 1)"), Ok(2));
    assert_eq!(expression("(- 6 2)"), Ok(4));
    assert_eq!(expression("(* 3 4)"), Ok(12));
    assert_eq!(expression("(/ 5 2)"), Ok(2));
    assert_eq!(expression("(/ (+ 1 1) 2)"), Ok(1));
    assert_eq!(expression("(/ (+ (- 4 1) 1) 2)"), Ok(2));
    assert_eq!(expression("(* (+ 1 1) (+ 2 2))"), Ok(8));
    println!("Ok");
}
