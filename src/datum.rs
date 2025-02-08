use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Datum {
    Bool(bool),
    Text(String),
    Num(i64),
    List(Vec<Datum>),
}

impl Display for Datum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Datum::Bool(x) => write!(f, "{:?}", x),
            Datum::Text(x) => write!(f, "{:?}", x),
            Datum::Num(x) => write!(f, "{:?}", x),
            Datum::List(vec) => write!(f, "[{:?}]", vec.iter().format(", ")),
        }
    }
}

impl Datum {
    pub fn from(raw: &str) -> Self {
        if raw.is_empty() {
            return Datum::Bool(true);
        }
        if raw == "LIST" {
            return Datum::List(Vec::new());
        }
        if raw.parse::<f64>().is_ok() {
            return Datum::Num(raw.parse::<i64>().unwrap_or(0));
        }
        return Datum::Text(raw.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let bool_value = Datum::from("");
        let str_value = Datum::from("Hello");
        let num_value = Datum::from("12");
        let list_value = Datum::from("LIST");

        assert!(bool_value == Datum::Bool(true));
        assert!(str_value == Datum::Text(String::from("Hello")));
        assert!(num_value == Datum::Num(12));
        assert!(list_value == Datum::List(Vec::new()));
    }
}
