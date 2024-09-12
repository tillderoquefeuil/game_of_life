use anyhow::Result;
use std::time::{Duration, Instant};
use crossbeam::{channel, select};
use crossterm::event::KeyCode;
use clap::Parser;

use crossterm::terminal;

mod board;
use board::*;

mod term;
use term::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Width of the board, default is the terminal width
    #[arg(long)]
    width: Option<usize>,
    /// Height of the board, default is the terminal height
    #[arg(long)]
    height: Option<usize>,
    /// Maximum width of the board, default is the terminal width
    #[arg(long)]
    max_width: Option<usize>,
    /// Maximum height of the board, default is the terminal height
    #[arg(long)]
    max_height: Option<usize>,
    /// Probability of initial cell to be alive
    #[arg(short, long, default_value_t = 0.5)]
    probability: f32,
    /// Delay between two generations, in milliseconds
    #[arg(short, long, default_value_t = 500.0)]
    delay: f64,
    /// Minimum neighbourg cells to born
    #[arg(long = "mi-b", default_value_t = 3)]
    min_to_born: u32,
    /// Maximum neighbourg cells to born
    #[arg(long = "ma-b", default_value_t = 3)]
    max_to_born: u32,
    /// Minimum neighbourg cells to stay alive
    #[arg(long = "mi-a", default_value_t = 2)]
    min_to_stay_alive: u32,
    /// Maximum neighbourg cells to stay alive
    #[arg(long = "ma-a", default_value_t = 3)]
    max_to_stay_alive: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    init_term()?;

    let term_size = terminal::size()?;

    let min_width = (term_size.0/2) as usize;
    let min_height = term_size.1 as usize - 3;

    let char_rx = read_char();

    let mut delay = args.delay;
    let width = args.width.unwrap_or(min_width);
    let max_width = args.max_width.unwrap_or(min_width);
    let height = args.height.unwrap_or(min_height);
    let max_height = args.max_height.unwrap_or(min_height);

    let mut board = Board::new(width, height)
        .with_max_width(max_width)
        .with_max_height(max_height)
        .with_min_to_born(args.min_to_born)
        .with_max_to_born(args.max_to_born)
        .with_min_to_stay_alive(args.min_to_stay_alive)
        .with_max_to_stay_alive(args.max_to_stay_alive)
        .with_random(args.probability);

    let mut play = true;
    let mut next = None;

    loop {
        let now = Instant::now();

        if next.is_none() {
            next = Some(Instant::now() + Duration::from_millis(delay as u64));
        }

        let timeout = channel::after(next.unwrap().saturating_duration_since(now));
        select! {
            recv(char_rx) -> key_code => {
                if let Ok(key_code) = key_code {
                    match key_code {
                        KeyCode::Esc => break,
                        KeyCode::Char('q') => break,
                        KeyCode::Left => delay = delay*1.25,
                        KeyCode::Right => delay = delay*0.75,
                        KeyCode::Char(' ') => play = !play,
                        _ => {
                            // println!("Key pressed: {:?}", key_code);
                        }
                    }
                }
            },
            recv(timeout) -> _ => {
                next = None;
                if play {
                    clear_terminal()?;
                    board.lines().iter().for_each(|line| {
                        println!("{}", line);
                        clear_line().unwrap();
                    });
                    jump_line()?;
                    println!("Press SPACE to toggle play/pause, LEFT/RIGHT to change speed and ESC to quit");

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
