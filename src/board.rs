#![allow(dead_code)]

use std::usize;
use rand::Rng;

const MIN_TO_BORN: u32 = 3;
const MAX_TO_BORN: u32 = 3;
const MIN_TO_STAY_ALIVE: u32 = 2;
const MAX_TO_STAY_ALIVE: u32 = 3;

pub struct Board {
    current: Vec<Vec<bool>>,
    max_size: Option<usize>,
    min_to_born: u32,
    max_to_born: u32,
    min_to_stay_alive: u32,
    max_to_stay_alive: u32,
}

impl Board {
    pub fn new(
        size: usize,
    ) -> Self {

        if size < 2 {
            panic!("Board size has to be greater than 1");
        }

        Self {
            current: vec![vec![false; size]; size],
            max_size: None,
            min_to_born: MIN_TO_BORN,
            max_to_born: MAX_TO_BORN,
            min_to_stay_alive: MIN_TO_STAY_ALIVE,
            max_to_stay_alive: MAX_TO_STAY_ALIVE,
        }
    }

    pub fn with_min_to_born(mut self, min_to_born: u32) -> Self {
        self.min_to_born = min_to_born;
        self
    }
    
    pub fn with_max_to_born(mut self, max_to_born: u32) -> Self {
        self.max_to_born = max_to_born;
        self
    }
    
    pub fn with_min_to_stay_alive(mut self, min_to_stay_alive: u32) -> Self {
        self.min_to_stay_alive = min_to_stay_alive;
        self
    }
    
    pub fn with_max_to_stay_alive(mut self, max_to_stay_alive: u32) -> Self {
        self.max_to_stay_alive = max_to_stay_alive;
        self
    }

    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn with_random(mut self, prob: f32) -> Self{
        self.current.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|value| {
                *value = get_random_bool(prob);
            });
        });

        self
    }


    // ■ □
    pub fn lines(&self) -> Vec<String> {
        let mut board = Vec::new();
        self.current.iter().for_each(|row| {
            let mut line = String::new();
            row.iter().for_each(|value| {
                line.push_str(if *value {"■ "} else {"□ "});
            });
            board.push(line);
        });

        board
    }
    
    pub fn next(&mut self) -> bool {
        let mut next = vec![vec![false; self.width()]; self.height()];
        
        self.current.iter().enumerate().for_each(|(row_index, row )| {
            row.iter().enumerate().for_each(|(col_index, value )| {
                let neighbours = self.calc_neighbours(row_index, col_index);

                if *value && self.will_stay_alive(neighbours) {
                    next[row_index][col_index] = true;
                } else if !value && self.will_born(neighbours) {
                    next[row_index][col_index] = true;
                } else {
                    next[row_index][col_index] = false;
                }

            });
        });
        
        if self.current == next {
            return false;
        }

        self.current = next;
        self.resizing_board();

        true
    }

    pub fn set_cell(&mut self, row_index: usize, col_index: usize, value: bool) {
        self.current[row_index][col_index] = value;
    }

    fn width(&self) -> usize {
        self.current.first().unwrap().len()
    }
    fn height(&self) -> usize {
        self.current.len()
    }
    
    fn calc_neighbours(&self, row_index: usize, col_index: usize) -> u32 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let row_index = row_index as i32 + i;
                let col_index = col_index as i32 + j;
                if row_index < 0 || col_index < 0 {
                    continue;
                }
                let row_index = row_index as usize;
                let col_index = col_index as usize;
                if row_index >= self.current.len() || col_index >= self.current[row_index].len() {
                    continue;
                }
                if self.current[row_index][col_index] {
                    count += 1;
                }
            }
        }
        count
    }

    fn will_stay_alive(&self, neighbours: u32) -> bool {
        self.min_to_stay_alive <= neighbours && neighbours <= self.max_to_stay_alive
    }    
    fn will_born(&self, neighbours: u32) -> bool {
        self.min_to_born <= neighbours && neighbours <= self.max_to_born
    }

    fn resizing_board(&mut self) {
        let max_size = match self.max_size {
            Some(size) => size,
            None => usize::MAX,
        };

        if self.height() < max_size {
                // first row
            if self.current.first().unwrap().iter().any(|cell| *cell) {
                self.prepend_row();
            }
            // last row
            if self.current.last().unwrap().iter().any(|cell| *cell) {
                self.append_row();
            }
        }
        if self.width() < max_size {
            // first col
            if self.current.iter().map(|row| row.first().unwrap()).any(|cell| *cell) {
                self.prepend_col();
            }
            // last col
            if self.current.iter().map(|row| row.last().unwrap()).any(|cell| *cell) {
                self.append_col();
            }
        }
    }
    fn prepend_row(&mut self) {
        self.current.insert(0, vec![false; self.current[0].len()]);
    }
    fn append_row(&mut self) {
        self.current.push(vec![false; self.current[0].len()]);
    }
    fn prepend_col(&mut self) {
        for row in self.current.iter_mut() {
            row.insert(0, false);
        }
    }
    fn append_col(&mut self) {
        for row in self.current.iter_mut() {
            row.push(false);
        }
    }

}

fn get_random_bool(prob: f32) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100) < (prob*100.0) as i32
}