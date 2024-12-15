use std::io;
use std::io::Read;
use std::net::TcpListener;

use rstore::datum::Datum;

fn parse_input(store: &mut Datum, input: String) {
    let cmds: Vec<&str> = input.split("|").collect();
    for cmd in cmds {
        let parts: Vec<&str> = cmd.split(" ").collect();

        if parts[0] == "ECHO" {
            println!("{:?}", parts[1]);
        } else if parts[0] == "PUSH" {
            store.insert(parts[1]);
        } else if parts[0] == "VIEW" {
            println!("{}", store.view(0));
        } else if parts[0] == "REMOVE" {
            store.delete(parts[1]);
        } else {
            println!("Invalid command. Options: ECHO PUSH VIEW REMOVE");
        }
    }
}

fn main() -> io::Result<()> {
    let mut store = Datum::new();

    loop {
        let listener = TcpListener::bind("127.0.0.1:9876")?;

        for stream in listener.incoming() {
            let mut buffer: [u8; 1024] = [0; 1024];
            let size = stream.unwrap().read(&mut buffer).unwrap();
            parse_input(
                &mut store,
                String::from_utf8(buffer[0..size].to_vec()).unwrap().clone(),
            );
        }
    }
}
