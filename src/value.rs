use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::fmt;

use crate::Error;

#[derive(Debug, Clone)]
pub enum Value {
    FLOAT(f64),
    //NIL
}

#[allow(unreachable_patterns)]
impl Neg for Value {
    type Output = Result<Value, Error>;

    fn neg(self) -> Result<Value, Error> {
        match self {
            Value::FLOAT(x) => Ok(Value::FLOAT(-x)),
            _ => Err(Error::SIGNAL)
        }
    }
}

#[allow(unreachable_patterns)]
impl Add for Value {
    type Output = Result<Value, Error>;

    fn add(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a + b)),
            _ => Err(Error::SIGNAL)
        }
    }
}

#[allow(unreachable_patterns)]
impl Sub for Value {
    type Output = Result<Value, Error>;

    fn sub(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a - b)),
            _ => Err(Error::SIGNAL)
        }
    }
}

#[allow(unreachable_patterns)]
impl Mul for Value {
    type Output = Result<Value, Error>;

    fn mul(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a * b)),
            _ => Err(Error::SIGNAL)
        }
    }
}

#[allow(unreachable_patterns)]
impl Div for Value {
    type Output = Result<Value, Error>;

    fn div(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => {
                if b == 0.0 {
                    Err(Error::DIVIDE_BY_ZERO)
                } else {
                    Ok(Value::FLOAT(a / b))
                }
            },
            _ => Err(Error::SIGNAL)
        }
    }
}

#[allow(unreachable_patterns)]
impl Rem for Value {
    type Output = Result<Value, Error>;

    fn rem(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => {
                if b == 0.0 {
                    Err(Error::DIVIDE_BY_ZERO)
                } else {
                    Ok(Value::FLOAT(a % b))
                }
            },
            _ => Err(Error::SIGNAL)
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::FLOAT(x) => write!(f, "{}", x),
        }
    }
}