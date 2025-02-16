use crate::datum::Datum;

pub struct Store {
    pub(crate) data: Vec<(String, Datum)>,
    pub(crate) cmds: Vec<String>,
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
        if let Some((_, Datum::List(contents))) = self.data.iter_mut().find(|(k, _)| k == key) {
            contents.push(Datum::from(value));
        } else {
            self.data.push((key.to_string(), Datum::from(value)));
        }
    }

    // SHOW (print the whole store)
    pub fn show_store(&mut self, len: usize) -> String {
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
    pub fn show_key(&mut self, key: &str) -> String {
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
                if parts.len() > 2 {
                    ret_msg = format!("{}\n{}", ret_msg, self.show_key(parts[1]));
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
                    "Valid commands: ADD, DEL, SHOW, HELP

    ADD K V - set key K to value V. if K holds a list, append the value
    ADD K - set key K to True
    ADD K LIST - Set key K to an empty array
    SHOW - Show the contents of the store
    SHOW K - Show the value of key K
    DEL K - Delete key K
    DEL K I - if key K contains a list, remove element at index I
    ",
                ));
            } else {
                ret_msg = format!("{}\nIUnknown command: {}", ret_msg, parts[1]);
            }
        }
        return Some(ret_msg.trim().to_owned());
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
