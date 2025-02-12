#![allow(clippy::needless_return)]

pub mod datum;
pub mod store;

use crate::store::Store;

pub fn update_screen(store: &Store) -> String {
    let (cols, rows) = crossterm::terminal::size().unwrap();
    let msg_len = (cols / 2) - 3;

    let mut screen = String::new();
    screen += &String::from(
        "\x1B[7m".to_owned()
            + &" ".repeat(msg_len.into())
            + "rstore"
            + &" ".repeat(msg_len.into())
            + "\x1B[27m",
    );
    screen += "\n"; // In theory, shouldn't be needed
    screen += &"\u2513".repeat(cols);

    let cmds = store.cmds.iter().rev().take((rows - 2) / 2);
    let contents = store.data.iter().rev().take((rows - 2) / 2);

    for elem in cmds { store += format!("{:?}\n", elem); }
    screen += &"\u2513".repeat(cols);
    for elem in contents { store += format!("{:?}\n", elem); }

    return screen;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_add_to_store() {
//         let mut store = HashMap::new();
//         add_to_store(&mut store, "Foo", "Bar");
//         assert_eq!(store.get("Foo"), Some(&Datum::Text(String::from("Bar"))), "{:?}", store);
//
//         add_to_store(&mut store, "A bool", "");
//         assert!(store.get("A bool") == Some(&Datum::Bool(true)));
//
//         add_to_store(&mut store, "lst", "LIST");
//         assert!(store.get("lst") == Some(&Datum::List(Vec::new())));
//     }
//
//     #[test]
//     fn test_show_store() {
//         let mut store = HashMap::new();
//         add_to_store(&mut store, "Foo", "Bar");
//         add_to_store(&mut store, "A bool", "");
//         add_to_store(&mut store, "lst", "LIST");
//
//         // TODO: There's no way to define the order of the map currentl
//         let expected = String::from("Foo: \"Bar\"\\nA bool: true\\nlst: []\\n");
//         assert_eq!(show_store(&mut store, 69), expected);
//     }
//
//     // #[test]
//     // fn test_show_key() {}
//     //
//     // #[test]
//     // fn test_delete_from_list() {}
//     //
//     // #[test]
//     // fn test_delete_key() {}
//     //
//     // #[test]
//     // fn test_update_screen() {}
//
// }
