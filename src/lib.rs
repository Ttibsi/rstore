#![allow(clippy::needless_return)]

pub mod datum;
pub mod store;

use crate::store::Store;

pub fn update_screen(store: &Store) -> String {
    let (cols, rows) = crossterm::terminal::size().unwrap();
    let msg_len = (cols - 6) / 2;

    let mut screen = String::new();
    screen += &("\x1B[7m".to_owned()
        + &" ".repeat(msg_len.into())
        + "rstore"
        + &" ".repeat(msg_len.into())
        + "\x1B[27m");

    if cols % 2 == 1 {
        screen += " ";
    }
    screen += "\r\n";

    let row_count: usize = ((rows - 2) / 2).into();
    let cmds = store
        .cmds
        .iter()
        .rev()
        .map(|s| s.as_str())
        .chain(std::iter::repeat(""))
        .take(row_count);
    let contents = store.data.iter().rev().take(row_count);

    for elem in cmds {
        if elem.is_empty() {
            screen += "\r\n";
        } else {
            screen += &format!("{:?}\r\n", elem);
        }
    }
    screen += &"\u{2500}".repeat(cols.into());
    for elem in contents {
        screen += &format!("{:?}\r\n", elem);
    }

    return screen;
}

#[cfg(test)]
mod tests {
    use datum::Datum;

    use super::*;

    #[test]
    fn test_update_screen() {
        let my_store = Store {
            data: vec![
                ("Foo".to_string(), Datum::Bool(true)),
                ("Bar".to_string(), Datum::Num(69)),
            ],
            cmds: vec!["Foo".to_string(), "Bar".to_string(), "Baz".to_string()],
        };

        let rendered_lines: Vec<String> = update_screen(&my_store)
            .split("\n")
            .map(String::from)
            .collect();

        assert!(rendered_lines[0].contains("rstore"));
        assert!(rendered_lines[1].contains("Baz"));
        assert!(rendered_lines[2].contains("Bar"));
        assert!(rendered_lines[3].contains("Foo"));
    }
}
