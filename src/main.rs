use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

// TODO: unit tests
// TODO: Add  tui on server to view the store
fn main() -> io::Result<()> {
    let mut store = HashMap::new();

    loop {
        let listener = TcpListener::bind("127.0.0.1:9876")?;

        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap();
            let mut buffer: [u8; 1024] = [0; 1024];
            let size = stream.read(&mut buffer).unwrap();
            let msg = rstore::parse_input(
                &mut store,
                String::from_utf8(buffer[0..size].to_vec()).unwrap().clone(),
            );

            if let Some(message) = msg {
                let _ = stream.write(&message.into_bytes());
            }
        }
    }
}
