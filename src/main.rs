#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg_file! lisp("grammar.rustpeg");

fn main() {
    println!("Ok");
}
