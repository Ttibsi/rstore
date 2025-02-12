use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::time;

use crossterm::{
    event::{self, poll, read, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal, ExecutableCommand,
};

// TODO: unit tests
// TODO: Add  tui on server to view the store
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let _ = terminal::enable_raw_mode();
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut store = rstore::store::Store::new();

    loop {
        let listener = TcpListener::bind("127.0.0.1:9876")?;
        rstore::update_screen(&mut store);

        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap();
            let mut buffer: [u8; 1024] = [0; 1024];
            let size = stream.read(&mut buffer).unwrap();
            let msg =
                store.parse_input(String::from_utf8(buffer[0..size].to_vec()).unwrap().clone());

            if let Some(message) = msg {
                let _ = stream.write(&message.into_bytes());
            }
        }

        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        if poll(time::Duration::from_millis(100))? {
            let read_key = read();
            if let Ok(key) = read_key {
                match key {
                    event::Event::Key(key_event) => {
                        if key_event == ctrl_c {
                            break;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    let _ = terminal::disable_raw_mode();
    Ok(())
}
