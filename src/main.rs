use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::time;

use crossterm::{
    cursor,
    event::{self, poll, read, KeyCode, KeyModifiers},
    execute, style, terminal, ExecutableCommand,
};

// TODO: unit tests
// TODO: Add  tui on server to view the store
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let _ = terminal::enable_raw_mode();
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut store = rstore::store::Store::new();

    'eventloop: loop {
        let listener = TcpListener::bind("127.0.0.1:9876")?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(style::Print(rstore::update_screen(&store)))?;

        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap();
            let mut buffer: [u8; 1024] = [0; 1024];
            let size = stream.read(&mut buffer).unwrap();
            let msg =
                store.parse_input(String::from_utf8(buffer[0..size].to_vec()).unwrap().clone());

            if let Some(message) = msg {
                let _ = stream.write(&message.into_bytes());
            }

            if poll(time::Duration::from_millis(100))? {
                match read()? {
                    event::Event::Key(key_event) => {
                        stdout.execute(style::Print(format!("{:?}", key_event)))?;
                        if key_event.code == KeyCode::Char('c')
                            && key_event.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            break 'eventloop;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    let _ = terminal::disable_raw_mode();
    Ok(())
}
