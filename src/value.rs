use std::ops::{Add, Sub, Mul, Div, Rem, Neg, BitOr, BitAnd};
use std::fmt;

use crate::Error;

#[derive(Debug, Clone)]
pub enum Value {
    FLOAT(f64),
    BOOL(bool),
    NIL
}


impl Neg for Value {
    type Output = Result<Value, Error>;

    fn neg(self) -> Result<Value, Error> {
        match self {
            Value::FLOAT(x) => Ok(Value::FLOAT(-x)),
            _ => Err(Error::SIGNAL)
        }
    }
}


impl Add for Value {
    type Output = Result<Value, Error>;

    fn add(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a + b)),
            _ => Err(Error::SIGNAL)
        }
    }
}

impl BitOr for Value {
    type Output = Result<Value, Error>;

    fn bitor(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::BOOL(a), Value::BOOL(b)) => Ok(Value::BOOL(a | b)),
            _ => Err(Error::SIGNAL)
        }
    }
}

impl BitAnd for Value {
    type Output = Result<Value, Error>;

    fn bitand(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::BOOL(a), Value::BOOL(b)) => Ok(Value::BOOL(a & b)),
            _ => Err(Error::SIGNAL)
        }
    }
}


impl Sub for Value {
    type Output = Result<Value, Error>;

    fn sub(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a - b)),
            _ => Err(Error::SIGNAL)
        }
    }
}


impl Mul for Value {
    type Output = Result<Value, Error>;

    fn mul(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a * b)),
            _ => Err(Error::SIGNAL)
        }
    }
}


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
            Value::BOOL(x) => write!(f, "{}", x),
            Value::NIL => write!(f, ""),
        }
    }
}