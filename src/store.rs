use crate::datum::Datum;

pub struct Store {
    pub(crate) data: Vec<(String, Datum)>,
    pub(crate) cmds: Vec<String>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        return Self {
            data: Vec::new(),
            cmds: Vec::new(),
        };
    }

    // ADD K V (set k's value to v. If k has a list as it's value, append it)
    // ADD K (set v to True)
    // ADD K LIST (create an empty list as the value)
    pub fn add_to_store(&mut self, key: &str, value: &str) {
        // if key already exists and value is not a List, replace it's value
        // if key already exists and it's value is a list, append value to the list
        // otherwise, create a new key and set it's value to value
        // if let Some((x, _)) = my_vec.iter_mut().find(|(x, _)| *x == value_to_find) {
        //     *x = value;
        // }

        if let Some((_, Datum::List(contents))) = self.data.iter_mut().find(|(k, _)| k == key) {
            contents.push(Datum::from(value));
        } else if let Some((_x, y)) = self.data.iter_mut().find(|(x, _y)| *x == key) {
            *y = Datum::from(value);
        } else {
            self.data.push((key.to_string(), Datum::from(value)));
        }
    }

    // SHOW (print the whole store)
    pub fn show_store(&self, len: usize) -> String {
        let mut ret = String::new();

        for (key, value) in &self.data {
            ret.push_str(key);
            ret += ": ";
            let mut rhs = value.to_string().clone();
            if len > 0 {
                // rhs = rhs[0..len].to_string();
                rhs = rhs.chars().take(len).collect();
            }

            ret += &rhs;
            ret += "\n";
        }

        return ret;
    }

    // SHOW K (Show the value(s) of k)
    pub fn show_key(&self, key: &str) -> String {
        if let Some((_, value)) = self.data.iter().find(|(k, _)| k == key) {
            return format!("{}: {}", key, value);
        }

        return "".to_string();
    }

    // DEL K (Delete k and it's values)
    pub fn delete_key(&mut self, key: &str) {
        if let Some(pos) = self.data.iter().position(|(k, _)| k == key) {
            self.data.remove(pos);
        }
    }

    // DEL K V (If K is a list, delete the value v from it's list)
    pub fn delete_from_list(&mut self, key: &str, index: usize) {
        if let Some((_, Datum::List(contents))) = self.data.iter_mut().find(|(k, _)| k == key) {
            if index < contents.len() {
                contents.remove(index);
            }
        }
    }

    pub fn jsonify(&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }

    pub fn parse_input(&mut self, input: String) -> Option<String> {
        let cmds: Vec<&str> = input.split("|").collect();
        let mut ret_msg = String::new();

        for cmd in cmds {
            let parts: Vec<&str> = cmd
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim())
                .collect();

            self.cmds.push(parts.join(" "));

            if parts[0] == "ADD" {
                if parts.len() == 2 {
                    self.add_to_store(parts[1], "");
                } else {
                    self.add_to_store(parts[1], parts[2]);
                }

                ret_msg = format!("{}\n{} Updated", ret_msg, parts[1]);
            } else if parts[0] == "SHOW" {
                if parts.len() > 1 {
                    ret_msg = format!("{}\n{}", ret_msg, self.show_key(parts[1]));
                    continue;
                }
                ret_msg = format!("{}\n{}", ret_msg, self.show_store(0));
            } else if parts[0] == "DEL" {
                if parts.len() == 2 {
                    self.delete_key(parts[1]);
                    ret_msg = format!("{}\nkey {} removed", ret_msg, parts[1]);
                    continue;
                }

                if let Ok(index) = parts[2].parse::<usize>() {
                    self.delete_from_list(parts[1], index);
                    ret_msg = format!("{}\nkey {} updated", ret_msg, parts[1]);
                    continue;
                }

                ret_msg = format!("{}\nInvalid Key: {}", ret_msg, parts[1]);
            } else if parts[0] == "HELP" {
                return Some(String::from(
                    "Valid commands: ADD, DEL, SHOW, HELP, EXPORT

    ADD K V - set key K to value V. if K holds a list, append the value
    ADD K - set key K to True
    ADD K LIST - Set key K to an empty array
    SHOW - Show the contents of the store
    SHOW K - Show the value of key K
    DEL K - Delete key K
    DEL K I - if key K contains a list, remove element at index I
    EXPORT - print a JSON string form of the current contents to stdout.
    ",
                ));
            } else if parts[0] == "EXPORT" {
                ret_msg = self.jsonify();
            } else {
                ret_msg = format!("{}\nUnknown command: {}", ret_msg, parts[0]);
            }
        }
        return Some(ret_msg.trim().to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_store() {
        let store = Store::default();
        assert_eq!(store.data.len(), 0);
        assert_eq!(store.cmds.len(), 0);
    }

    #[test]
    fn test_new_store() {
        let store = Store::new();
        assert_eq!(store.data.len(), 0);
        assert_eq!(store.cmds.len(), 0);
    }

    #[test]
    fn test_add_to_store() {
        let mut store = Store::new();
        store.add_to_store("key1", "LIST");
        store.add_to_store("key1", "value1");
        assert_eq!(store.data.len(), 1);
        assert_eq!(store.data[0].0, "key1");
        assert_eq!(store.data[0].1, Datum::List(vec![Datum::from("value1")]));

        store.add_to_store("key1", "value2");
        assert_eq!(store.data.len(), 1);
        assert_eq!(store.data[0].0, "key1");
        assert_eq!(
            store.data[0].1,
            Datum::List(vec![Datum::from("value1"), Datum::from("value2")])
        );
    }

    #[test]
    fn test_show_store() {
        let mut store = Store::new();
        store.add_to_store("key1", "value1");
        store.add_to_store("key2", "value2");
        store.add_to_store("key3", "value3");

        let output = store.show_store(0);
        assert_eq!(
            output,
            "key1: \"value1\"\nkey2: \"value2\"\nkey3: \"value3\"\n"
        );

        let output = store.show_store(3);
        assert_eq!(output, "key1: \"va\nkey2: \"va\nkey3: \"va\n");
    }

    #[test]
    fn test_show_key() {
        let mut store = Store::new();
        store.add_to_store("key1", "value1");
        store.add_to_store("key2", "value2");

        let output = store.show_key("key1");
        assert_eq!(output, "key1: \"value1\"");

        let output = store.show_key("key3");
        assert_eq!(output, "");
    }

    #[test]
    fn test_delete_key() {
        let mut store = Store::new();
        store.add_to_store("key1", "value1");
        store.add_to_store("key2", "value2");

        store.delete_key("key1");
        assert_eq!(store.data.len(), 1);
        assert_eq!(store.data[0].0, "key2");

        store.delete_key("key3");
        assert_eq!(store.data.len(), 1);
    }

    #[test]
    fn test_delete_from_list() {
        let mut store = Store::new();
        store.add_to_store("key1", "LIST");
        store.add_to_store("key1", "value1");
        store.add_to_store("key1", "value2");
        store.add_to_store("key1", "value3");

        store.delete_from_list("key1", 1);
        assert_eq!(
            store.data[0].1,
            Datum::List(vec![Datum::from("value1"), Datum::from("value3")])
        );

        store.delete_from_list("key1", 10);
        assert_eq!(
            store.data[0].1,
            Datum::List(vec![Datum::from("value1"), Datum::from("value3")])
        );
    }
    #[test]
    fn test_jsonify() {
        let mut store = Store::new();
        store.add_to_store("key1", "value1");
        store.add_to_store("key2", "value2");

        let json_output = store.jsonify();
        assert_eq!(
            json_output,
            r#"[["key1",{"Text":"value1"}],["key2",{"Text":"value2"}]]"#
        );
    }

    #[test]
    fn test_parse_input() {
        let mut store = Store::new();

        let input = "ADD key1 value1 | ADD key2 value2";
        let output = store.parse_input(input.to_string()).unwrap();
        assert_eq!(output, "key1 Updated\nkey2 Updated");

        let input = "SHOW";
        let output = store.parse_input(input.to_string()).unwrap();
        assert!(output.contains("key1: \"value1\""));
        assert!(output.contains("key2: \"value2\""));

        let input = "SHOW key1";
        let output = store.parse_input(input.to_string()).unwrap();
        assert_eq!(output, "key1: \"value1\"");

        let input = "DEL key1";
        let output = store.parse_input(input.to_string()).unwrap();
        assert_eq!(output, "key key1 removed");

        let input = "DEL key2 1";
        let output = store.parse_input(input.to_string()).unwrap();
        assert_eq!(output, "key key2 updated");

        let input = "INVALID_COMMAND";
        let output = store.parse_input(input.to_string()).unwrap();
        assert_eq!(output, "Unknown command: INVALID_COMMAND");

        let input = "HELP";
        let output = store.parse_input(input.to_string()).unwrap();
        assert!(output.contains("Valid commands: ADD, DEL, SHOW, HELP, EXPORT"));
    }
}
