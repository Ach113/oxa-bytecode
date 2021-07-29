use std::ops::{Add, Sub, Mul, Div, Rem, Neg};
use std::fmt;

use crate::Error;

#[derive(Debug, Clone)]
pub enum Value {
    FLOAT(f64),
    //NIL
}

impl Neg for Value {
    type Output = Result<Value, Error>;

    fn neg(self) -> Result<Value, Error> {
        match self {
            Value::FLOAT(x) => Ok(Value::FLOAT(-x)),
            _ => Err(Error::STRING("TypeError for operation `neg`".to_string()))
        }
    }
}

impl Add for Value {
    type Output = Result<Value, Error>;

    fn add(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a + b)),
            _ => Err(Error::STRING("TypeError for operation `+`".to_string()))
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, Error>;

    fn sub(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a - b)),
            _ => Err(Error::STRING("TypeError for operation `-`".to_string()))
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, Error>;

    fn mul(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => Ok(Value::FLOAT(a * b)),
            _ => Err(Error::STRING("TypeError for operation `*`".to_string()))
        }
    }
}

impl Div for Value {
    type Output = Result<Value, Error>;

    fn div(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => {
                if b == 0.0 {
                    Err(Error::STRING("ZeroDivisionError".into()))
                } else {
                    Ok(Value::FLOAT(a / b))
                }
            },
            _ => Err(Error::STRING("TypeError for operator /".into()))
        }
    }
}

impl Rem for Value {
    type Output = Result<Value, Error>;

    fn rem(self, right: Value) -> Result<Value, Error> {
        match (self, right) {
            (Value::FLOAT(a), Value::FLOAT(b)) => {
                if b == 0.0 {
                    Err(Error::STRING("ZeroDivisionError".into()))
                } else {
                    Ok(Value::FLOAT(a % b))
                }
            },
            _ => Err(Error::STRING("TypeError for operator /".into()))
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