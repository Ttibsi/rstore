#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Datum {
    Null,
    Text(String),
    Num(i64),
    List(Vec<Datum>),
    Pair { left: Box<Datum>, right: Box<Datum> },
}

impl Datum {
    pub fn new() -> Self {
        Datum::List(Vec::new())
    }

    pub fn parse(value: &str) -> Self {
        if value == "None" {
            return Datum::Null;
        }
        if value == "Null" {
            return Datum::Null;
        }
        if let Ok(ret) = value.parse::<i64>() {
            return Datum::Num(ret);
        }

        // Create List
        let mut chars = value.chars();
        if chars.next().unwrap() == '[' && chars.next_back().unwrap() == ']' {
            let tokens = value[1..value.len() - 1].split(",").collect::<Vec<&str>>();
            let mut lst: Vec<Self> = Vec::new();
            for tok in tokens {
                lst.push(Datum::parse(tok));
            }

            return Datum::List(lst);
        }

        // Create Pair
        let parts: Vec<&str> = value.split(":").collect();
        if parts.len() == 2 {
            return Datum::Pair {
                left: Box::new(Datum::parse(parts[0])),
                right: Box::new(Datum::parse(parts[1])),
            };
        }

        Datum::Text(value.to_string())
    }

    pub fn insert(&mut self, value: &str) {
        if value.contains("::") {
            let (key, rest) = value.split_once("::").unwrap();

            match self {
                Datum::List(vec) => vec[key.parse::<usize>().unwrap()].insert(rest),
                Datum::Pair { left, right } => {
                    if **left == Datum::parse(key) {
                        right.insert(rest);
                    }
                }
                _ => (),
            }
        } else {
            match self {
                Datum::List(vec) => vec.push(Datum::parse(value)),
                Datum::Pair { left: _, right } => *right = Box::new(Datum::parse(value)),
                _ => (),
            }
        }
    }

    pub fn delete(&mut self, value: &str) {
        if value.contains("::") {
            let (key, rest) = value.split_once("::").unwrap();

            match self {
                Datum::List(vec) => vec[key.parse::<usize>().unwrap()].delete(rest),
                Datum::Pair { left, right } => {
                    if **left == Datum::parse(key) {
                        right.delete(rest);
                    }
                }
                _ => (),
            }
        } else {
            match self {
                Datum::List(vec) => _ = vec.remove(value.parse::<usize>().unwrap()),
                Datum::Pair { left: _, right } => *right = Box::new(Datum::Null),
                _ => (),
            };
        }
    }

    pub fn view(&self, indent: usize) -> String {
        let tab = " ".repeat(indent * 4);

        match self {
            Datum::Null => format!("{}Null", tab).to_string(),
            Datum::Text(elem) => format!("{}{}", tab, elem).to_string(),
            Datum::Num(elem) => format!("{}{}", tab, elem).to_string(),
            Datum::List(vec) => {
                let mut str = format!("{}[\n", tab);
                for (idx, elem) in vec.iter().enumerate() {
                    str += &elem.view(indent + 1);

                    if idx != vec.len() - 1 {
                        str += ",\n";
                    } else {
                        str += "\n";
                    }
                }
                str += &format!("{}]", tab);
                str
            }
            Datum::Pair { left, right } => {
                let mut str = format!("{}(\n", tab);
                str += &left.view(indent + 1);
                str += ",\n";
                str += &right.view(indent + 1);
                str += &format!("\n{})", tab);

                str
            }
        }
    }
}

impl Default for Datum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut elem = Datum::new();
        elem.insert("5");

        assert_eq!(elem.view(0), "[\n    5\n]");
    }

    #[test]
    fn test_delete() {
        let mut elem = Datum::new();
        elem.insert("5");
        assert_eq!(elem.view(0), "[\n    5\n]");

        elem.delete("0");
        assert_eq!(elem.view(0), "[\n]");
    }

    #[test]
    fn test_view() {
        let mut elem = Datum::new();
        assert_eq!(elem.view(0), "[\n]");

        elem.insert("5");
        assert_eq!(elem.view(0), "[\n    5\n]");

        elem.delete("0");
        assert_eq!(elem.view(0), "[\n]");
    }

    #[test]
    fn test_types_parse_value() {
        assert_eq!(Datum::parse("None"), Datum::Null);
        assert_eq!(Datum::parse("1"), Datum::Num(1));
        assert_eq!(Datum::parse("foo"), Datum::Text("foo".to_string()));
    }
}
