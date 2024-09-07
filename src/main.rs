use anyhow::Result;
use std::time::{Duration, Instant};

use crossbeam::{channel, select};
use crossterm::event::KeyCode;

mod board;
use board::*;

mod term;
use term::*;

const SIZE: usize = 50;
const MAX_SIZE: usize = 50;

fn main() -> Result<()> {
    init_term()?;

    let char_rx = read_char();

    let mut board = Board::new(SIZE)
        .with_max_size(MAX_SIZE)
        .with_random();

    let mut speed = 500.0;
    let mut play = true;
    let mut next = None;

    loop {
        let now = Instant::now();

        if next.is_none() {
            next = Some(Instant::now() + Duration::from_millis(speed as u64));
        }

        let delay = channel::after(next.unwrap().saturating_duration_since(now));
        select! {
            recv(char_rx) -> key_code => {
                if let Ok(key_code) = key_code {
                    match key_code {
                        KeyCode::Esc => break,
                        KeyCode::Char('q') => break,
                        KeyCode::Left => speed = speed*1.25,
                        KeyCode::Right => speed = speed*0.75,
                        KeyCode::Char(' ') => play = !play,
                        _ => {
                            // println!("Key pressed: {:?}", key_code);
                        }
                    }
                }
            },
            recv(delay) -> _ => {
                next = None;
                if play {
                    clear_terminal()?;
                    board.lines().iter().for_each(|line| {
                        println!("{}", line);
                        clear_line().unwrap();
                    });
                    jump_line()?;
                    println!("Press SPACE to toggle play/pause, LEFT to slow down, RIGHT to speed up and ESC to quit");

                    if !board.next() {
                        break;
                    }
                }
            },
        }
    }
    
    clear_terminal()?;
    board.lines().iter().for_each(|line| {
        println!("{}", line);
        clear_line().unwrap();
    });
    jump_line()?;
    println!("Game Over ✨");

    reset_term()?;
    Ok(())
}
