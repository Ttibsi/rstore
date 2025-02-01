pub mod datum;

use crate::datum::Datum;
use std::collections::HashMap;

// ADD K V (set k's value to v. If k has a list as it's value, append it)
// ADD K (set v to True)
// ADD K LIST (create an empty list as the value)
fn add_to_store(store: &mut HashMap<String, Datum>, key: &str, value: &str) {
    if let Some(Datum::List(contents)) = store.get_mut(key) {
        contents.push(Datum::from(value));
    } else {
        store.insert(key.to_string(), Datum::from(value));
    }
}

// SHOW (print the whole store)
fn show_store(store: &mut HashMap<String, Datum>) -> String {
    let mut ret = String::new();

    store.iter_mut().for_each(|(key, value)| {
        ret += key;
        ret += ": ";
        ret += &value.to_string().clone();
        ret += "\n";
    });

    return ret;
}

// SHOW K (Show the value(s) of k)
fn show_key(store: &HashMap<String, Datum>, key: &str) -> String {
    if store.contains_key(key) {
        return format!("{}: {}", key, store[key]);
    }

    return "".to_string();
}

// DEL K (Delete k and it's values)
fn delete_key(store: &HashMap<String, Datum>, key: &str) {
    todo!();
}

// DEL K V (If K is a list, delete the value v from it's list)
fn delete_from_list(store: &HashMap<String, Datum>, key: &str, value: &str) {
    todo!();
}

pub fn parse_input(store: &mut HashMap<String, Datum>, input: String) -> Option<String> {
    let cmds: Vec<&str> = input.split("|").collect();
    for cmd in cmds {
        let parts: Vec<&str> = cmd.split(" ").collect();

        if parts[0] == "ADD" {
            if parts.len() == 2 {
                add_to_store(store, parts[1], "");
            }
            add_to_store(store, parts[1], parts[2]);

            return Some(format!("{} Updated", parts[1]));
        } else if parts[0] == "SHOW" {
            if parts.len() > 1 {
                return Some(show_key(store, parts[1]));
            }
            return Some(show_store(store));
        } else if parts[0] == "DEL" {
            if parts.len() == 2 {
                delete_key(store, parts[1]);
            }
            delete_from_list(store, parts[1], parts[2]);
        } else {
            return Some(String::from("Unknown command: ") + parts[0]);
        }
    }
    None
}
