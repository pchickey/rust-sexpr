#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod ast;
use ast::Sexpr;

use sexpr_grammar::*;
peg_file! sexpr_grammar("sexpr.rustpeg");


pub fn parse(s : &str) -> Result<ast::Sexpr, sexpr_grammar::ParseError> {
    return sexpr_grammar::start(s);
}


pub fn main() {
    println!("{:?}", Sexpr::List(Box::new(vec![])));
    println!("{:?}", Sexpr::Integer(0xdeadbeef));
    println!("{:?}", Sexpr::Float(3.14159));
    println!("{:?}", Sexpr::List(Box::new(vec![ Sexpr::Float(1.2345),
                                                Sexpr::Integer(67890)])));
}

#[cfg(test)]

mod test {

    use super::ast::Sexpr;
    use super::parse;
    use sexpr_grammar;

    fn list_vec(v : Vec<Sexpr>) -> Sexpr {
        return Sexpr::List(Box::new(v));
    }

    fn list_singleton(e: Sexpr) -> Sexpr {
        return list_vec(vec![e]);
    }

    #[test]
    fn test_atoms() {
        assert_eq!(sexpr_grammar::atom("0"), Ok(Sexpr::Integer(0)));
        assert_eq!(sexpr_grammar::atom("1"), Ok(Sexpr::Integer(1)));
        assert_eq!(sexpr_grammar::atom("10"), Ok(Sexpr::Integer(10)));
        assert_eq!(sexpr_grammar::atom("-1"), Ok(Sexpr::Integer(-1)));
        assert_eq!(sexpr_grammar::atom("0.0"), Ok(Sexpr::Float(0.0)));
        assert_eq!(sexpr_grammar::atom("+1.0"), Ok(Sexpr::Float(1.0)));
        assert_eq!(sexpr_grammar::atom("-0.0"), Ok(Sexpr::Float(0.0)));
        assert_eq!(sexpr_grammar::atom("-1.0"), Ok(Sexpr::Float(-1.0)));

        assert!(sexpr_grammar::atom("00000").is_err());
        assert!(sexpr_grammar::atom("01").is_err());
        assert!(sexpr_grammar::atom("01.1").is_err());

    }

    #[test]
    fn test_lists() {
        assert_eq!(parse("(0.0)")
                   , Ok(list_singleton(Sexpr::Float(0.0))));
        assert_eq!(parse("( 1.0)")
                   , Ok(list_singleton(Sexpr::Float(1.0))));
        assert_eq!(parse("(2.0 )")
                   , Ok(list_singleton(Sexpr::Float(2.0))));
        assert_eq!(parse("(1 2 3)")
                   , Ok(list_vec(vec![ Sexpr::Integer(1)
                                     , Sexpr::Integer(2)
                                     , Sexpr::Integer(3)
                                     ])));


        assert_eq!(parse("((1))")
                   , Ok(list_singleton(list_singleton(Sexpr::Integer(1)))));

        assert_eq!(parse("(((1)))")
                   , Ok(list_singleton(list_singleton(list_singleton(Sexpr::Integer(1))))));

        assert_eq!(parse("( ((1 )))")
                   , Ok(list_singleton(list_singleton(list_singleton(Sexpr::Integer(1))))));

        assert_eq!(parse("((1) 2 3)")
                   , Ok(list_vec(vec![ list_singleton(Sexpr::Integer(1))
                                     , Sexpr::Integer(2)
                                     , Sexpr::Integer(3)
                                     ])));

        assert!(parse("").is_err());
        assert!(parse("(").is_err());
        assert!(parse("1").is_err());
        assert!(parse("(()").is_err());
        assert!(parse("())").is_err());
    }

}

#[test]
fn some_test() {
    main();
}
