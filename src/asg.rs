#[derive(Debug)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl From<i64> for Number {
    fn from(val: i64) -> Number {
        Number::Int(val)
    }
}

impl From<f64> for Number {
    fn from(val: f64) -> Number {
        Number::Float(val)
    }
}
