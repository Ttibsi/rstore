pub mod datum;

use crate::datum::Datum;
use std::collections::HashMap;

// ADD K V (set k's value to v. If k has a list as it's value, append it)
// ADD K (set v to True)
// ADD K LIST (create an empty list as the value)
// SHOW (print the whole store)
// SHOW K (Show the value(s) of k)
// DEL K (Delete k and it's values)
// DEL K V (If K is a list, delete the value v from it's list)
//

pub fn parse_input(store: &mut HashMap<String, Datum>, input: String) -> Option<String> {
    let cmds: Vec<&str> = input.split("|").collect();
    for cmd in cmds {
        let parts: Vec<&str> = cmd.split(" ").collect();

        if parts[0] == "ADD" {
        } else if parts[0] == "SHOW" {
            let mut ret = String::new();

            store.into_iter().for_each(|(key, value)| {
                ret += key;
                ret += ": ";
                ret += &value.to_string().clone();
                ret += "\n";
            });

            return Some(ret);
        } else if parts[0] == "DEL" {
        } else {
            return Some(String::from("Unknown command: ") + parts[0]);
        }

        // if parts[0] == "ECHO" {
        //     println!("{:?}", parts[1]);
        // } else if parts[0] == "PUSH" {
        //     store.insert(parts[1]);
        // } else if parts[0] == "VIEW" {
        //     println!("{}", store.view(0));
        // } else if parts[0] == "REMOVE" {
        //     store.delete(parts[1]);
        // } else {
        //     println!("Invalid command. Options: ECHO PUSH VIEW REMOVE");
        // }
    }

    None
}
