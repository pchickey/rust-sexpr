#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate sexpr;

use sexpr::Sexpr;

use sexpr_grammar::*;
peg_file! sexpr_grammar("sexpr.rustpeg");


fn main() {

    println!("{:?}", Sexpr::List(vec![]));

    assert_eq!(sexpr_grammar::start("0"), Ok(Sexpr::Integer(0)));
    assert_eq!(sexpr_grammar::start("0.0"), Ok(Sexpr::Float(0.0)));

    //assert!(sexpr::start("").is_err());
}
