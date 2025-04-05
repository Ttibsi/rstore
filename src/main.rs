use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::time;

use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{
    cursor,
    event::{self, poll, read, KeyCode},
    execute, style, terminal, ExecutableCommand,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let _ = terminal::enable_raw_mode();
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let store = Arc::new(Mutex::new(rstore::store::Store::new()));
    let store_clone = Arc::clone(&store);

    // let socket_thread = thread::spawn(move || {
    let _ = thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:9876").unwrap();
        for stream_result in listener.incoming() {
            let mut stream = stream_result.unwrap();
            let mut buffer: [u8; 1024] = [0; 1024];
            let size = stream.read(&mut buffer).unwrap();
            let msg = {
                let mut store = store_clone.lock().unwrap();
                store.parse_input(String::from_utf8(buffer[0..size].to_vec()).unwrap().clone())
            };

            if let Some(mut message) = msg {
                message += "\n";
                let _ = stream.write(&message.into_bytes());
            }
        }
    });

    'eventloop: loop {
        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(style::Print(rstore::update_screen(&store.lock().unwrap())))?;

        if poll(time::Duration::from_millis(100))? {
            match read()? {
                event::Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        break 'eventloop;
                    }
                }
                _ => continue,
            }
        }
    }

    // socket_thread.join().unwrap();
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    let _ = terminal::disable_raw_mode();
    Ok(())
}
