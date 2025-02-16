use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    fn test_datum_display() {
        assert_eq!(Datum::Bool(true).to_string(), "true");
        assert_eq!(Datum::Text("hello".to_string()).to_string(), "\"hello\"");
        assert_eq!(Datum::Num(42).to_string(), "42");
        assert_eq!(
            Datum::List(vec![
                Datum::Bool(true),
                Datum::Text("test".to_string()),
                Datum::Num(123)
            ])
            .to_string(),
            "[Bool(true), Text(\"test\"), Num(123)]"
        );
    }

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
