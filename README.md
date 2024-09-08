# Game of Life CLI

A command-line implementation of Conway's Game of Life, written in Rust. This simulation runs on a customizable board with user-defined rules and options.

## Table of Contents
- [Usage](#usage)
- [Installation](#installation)
- [Options](#options)
- [Example](#example)

## Usage

Run the game using the following command:

```bash
game_of_life [OPTIONS]
```

## Installation

To install and run the Game of Life CLI, make sure you have Rust installed. Then, you can build the project using Cargo:

```bash
git clone https://github.com/tillderoquefeuil/game_of_life.git
cd game_of_life
cargo build --release
./target/release/game_of_life [OPTIONS]
```

## Options

Here are the available options to customize the Game of Life simulation:

- `-s, --size <SIZE>`: Size of the board (default: terminal size).
- `--max-size <MAX_SIZE>`: Maximum size of the board (default: terminal size).
- `-p, --probability <PROBABILITY>`: Probability for an initial cell to be alive. The default value is `0.5`.
- `-d, --delay <DELAY>`: Delay between two generations, in milliseconds (default: `500` ms).
- `--mi-b <MIN_TO_BORN>`: Minimum number of neighbor cells required for a new cell to be born (default: `3`).
- `--ma-b <MAX_TO_BORN>`: Maximum number of neighbor cells required for a new cell to be born (default: `3`).
- `--mi-a <MIN_TO_STAY_ALIVE>`: Minimum number of neighbor cells required for a cell to stay alive (default: `2`).
- `--ma-a <MAX_TO_STAY_ALIVE>`: Maximum number of neighbor cells required for a cell to stay alive (default: `3`).
- `-h, --help`: Prints the help message.
- `-V, --version`: Prints the version of the program.

## Example

Run the game with a custom board size, set the probability for cells to start alive, and adjust the rules for cell survival and birth:

```bash
game_of_life --size 25 --probability 0.7 --delay 300 --mi-b 2 --ma-b 3 --mi-a 1 --ma-a 4
```

In this example:
- The board size is 25.
- 70% of the cells will be initially alive.
- The delay between each generation is 300 ms.
- A cell is born if it has 2 or 3 neighboring cells.
- A cell stays alive with 1 to 4 neighboring cells.
