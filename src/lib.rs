
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Sexpr {
    List(Vec<Box<Sexpr>>),
    Integer(i64),
    Float(f64)
}


#[test]
fn main() {

    //assert_eq!(sexpr::start("0"), Ok(0));

    //assert!(sexpr::start("").is_err());
}
