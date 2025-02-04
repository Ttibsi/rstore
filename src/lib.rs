#![allow(clippy::needless_return)]

pub mod datum;

use crate::datum::Datum;
use std::collections::HashMap;

use crossterm::style::Stylize;

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
fn show_store(store: &mut HashMap<String, Datum>, len: usize) -> String {
    let mut ret = String::new();

    store.iter_mut().for_each(|(key, value)| {
        ret += key;
        ret += ": ";
        let mut rhs = value.to_string().clone();
        if len > 0 {
            rhs = rhs[0..len].to_string();
        }

        ret += &rhs;
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
fn delete_key(store: &mut HashMap<String, Datum>, key: &str) {
    store.remove(key);
}

// DEL K V (If K is a list, delete the value v from it's list)
fn delete_from_list(store: &mut HashMap<String, Datum>, key: &str, index: usize) {
    if let Some(Datum::List(contents)) = store.get_mut(key) {
        contents.remove(index);
    }
}

pub fn parse_input(store: &mut HashMap<String, Datum>, input: String) -> Option<String> {
    let cmds: Vec<&str> = input.split("|").collect();
    let mut ret_msg = String::new();

    for cmd in cmds {
        let parts: Vec<&str> = cmd
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .collect();

        println!("{:?}", parts);

        if parts[0] == "ADD" {
            if parts.len() == 2 {
                add_to_store(store, parts[1], "");
            } else {
                add_to_store(store, parts[1], parts[2]);
            }

            ret_msg = format!("{}\n{} Updated", ret_msg, parts[1]);
        } else if parts[0] == "SHOW" {
            if parts.len() > 2 {
                ret_msg = format!("{}\n{}", ret_msg, show_key(store, parts[1]));
            }
            ret_msg = format!("{}\n{}", ret_msg, show_store(store, 0));
        } else if parts[0] == "DEL" {
            if parts.len() == 2 {
                delete_key(store, parts[1]);
                ret_msg = format!("{}\nkey {} removed", ret_msg, parts[1]);
                continue;
            }

            if let Ok(index) = parts[2].parse::<usize>() {
                delete_from_list(store, parts[1], index);
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

pub fn update_screen(store: &mut HashMap<String, Datum>) -> String {
    let (cols, rows) = crossterm::terminal::size().unwrap();
    let msg_len = cols / 2;

    let mut screen = String::new();
    screen += &show_store(store, rows.into());

    return screen;
}
