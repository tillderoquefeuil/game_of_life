use anyhow::Result;
use std::io::{self, Write};
use std::time::Duration;

use crossbeam::channel;
use crossterm::{
    cursor,
    terminal,
    event::{self, Event, KeyCode},
    ExecutableCommand
};

pub fn read_char() -> channel::Receiver<KeyCode> {
    let (char_tx, char_rx) = channel::bounded(10);

    std::thread::spawn(move || {
        loop {
            if let Ok(_) = event::poll(Duration::from_millis(50)) {
                if let Ok(Event::Key(key_event)) = event::read() {
                    char_tx.send(key_event.code).unwrap();
                }
            }
    
        }
    });

    char_rx
}

pub fn init_term() -> Result<()> {
    terminal::enable_raw_mode()?;
    io::stdout().execute(cursor::Hide)?;
    io::stdout().flush()?;
    Ok(())
}

pub fn reset_term() -> Result<()> {
    io::stdout().execute(cursor::MoveToColumn(0))?;
    io::stdout().execute(cursor::Show)?;
    io::stdout().flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn clear_terminal() -> Result<()> {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    io::stdout().execute(cursor::MoveTo(0,0))?;
    io::stdout().flush()?;
    Ok(())
}

pub fn clear_line() -> Result<()> {
    io::stdout().execute(cursor::MoveToColumn(0))?;
    io::stdout().flush()?;
    Ok(())
}

pub fn jump_line() -> Result<()> {
    io::stdout().execute(cursor::MoveToNextLine(1))?;
    io::stdout().flush()?;
    Ok(())
}