
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Sexpr {
    List(Box<Vec<Sexpr>>),
    Integer(i64),
    Float(f64)
}

