#[cfg(test)]
mod tests {
    use crate::value::Value;
    use crate::*;

    #[test]
    fn variable_tests() -> Result<(), Error> {
        assert_eq!(Value::NIL, crate::interpret("var a; a;".to_string())?); // variable initializes to nil
        assert_eq!(Value::FLOAT(5.0), crate::interpret("var a = 5; a;".to_string())?);
        assert_eq!(Value::FLOAT(5.0), crate::interpret("var a; a = 5; a;".to_string())?);
        Ok(())
    }
}