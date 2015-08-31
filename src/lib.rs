#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod ast;
use ast::Sexpr;

use sexpr_grammar::*;
peg_file! sexpr_grammar("sexpr.rustpeg");


fn list_vec(v : Vec<Sexpr>) -> Sexpr {
    return Sexpr::List(Box::new(v));
}

fn list_singleton(e: Sexpr) -> Sexpr {
    return list_vec(vec![e]);
}

pub fn main() {
    println!("{:?}", Sexpr::List(Box::new(vec![])));
    println!("{:?}", Sexpr::Integer(0xdeadbeef));
    println!("{:?}", Sexpr::Float(3.14159));
    println!("{:?}", Sexpr::List(Box::new(vec![ Sexpr::Float(1.2345),
                                                Sexpr::Integer(67890)])));

    assert_eq!(sexpr_grammar::atom("0"), Ok(Sexpr::Integer(0)));
    assert_eq!(sexpr_grammar::atom("0.0"), Ok(Sexpr::Float(0.0)));

    assert_eq!(sexpr_grammar::start("(0.0)")
               , Ok(list_singleton(Sexpr::Float(0.0))));
    assert_eq!(sexpr_grammar::start("( 1.0)")
               , Ok(list_singleton(Sexpr::Float(1.0))));
    assert_eq!(sexpr_grammar::start("(2.0 )")
               , Ok(list_singleton(Sexpr::Float(2.0))));
    assert_eq!(sexpr_grammar::start("(1 2 3)")
               , Ok(list_vec(vec![ Sexpr::Integer(1)
                                 , Sexpr::Integer(2)
                                 , Sexpr::Integer(3)
                                 ])));


    assert_eq!(sexpr_grammar::start("((1))")
               , Ok(list_singleton(list_singleton(Sexpr::Integer(1)))));

    assert_eq!(sexpr_grammar::start("(((1)))")
               , Ok(list_singleton(list_singleton(list_singleton(Sexpr::Integer(1))))));

    assert_eq!(sexpr_grammar::start("( ((1 )))")
               , Ok(list_singleton(list_singleton(list_singleton(Sexpr::Integer(1))))));

    assert_eq!(sexpr_grammar::start("((1) 2 3)")
               , Ok(list_vec(vec![ list_singleton(Sexpr::Integer(1))
                                 , Sexpr::Integer(2)
                                 , Sexpr::Integer(3)
                                 ])));

    assert!(sexpr_grammar::start("").is_err());
    assert!(sexpr_grammar::start("(").is_err());
    assert!(sexpr_grammar::start("1").is_err());
    assert!(sexpr_grammar::start("(()").is_err());
    assert!(sexpr_grammar::start("())").is_err());
}

#[test]
fn some_test() {
    main();
}
