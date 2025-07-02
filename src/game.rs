use crate::board::{Board, BoardUpdateError};
use crate::{board, play};
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(PartialEq)]
enum GameState {
    Running,
    WillExit,
    Won,
}

pub struct Game {
    state: GameState,
    board: Board,
    move_count: u32,
    players: HashMap<u32, String>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            state: GameState::Running,
            board: Board::default(),
            move_count: 0,
            players: HashMap::new(),
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        println!("> Welcome to the game! Enter 'q' to quit.");

        loop {
            match self.state {
                GameState::WillExit => break,
                _ => self.update(),
            }
        }
    }

    pub fn update(&mut self) {
        println!("{}", self.board);

        let player_number = self.move_count % 2 + 1;

        if !self.players.contains_key(&player_number) {
            let name = Game::ask(format!("Player {}, what's your name?: ", player_number).as_str());
            self.players.insert(player_number, name.clone());
        }

        let player_name = self.players.entry(player_number).or_default();

        let next_tile_state = match self.move_count % 2 {
            0 => board::TileState::Ex,
            _ => board::TileState::Oh,
        };

        if self.state == GameState::Won {
            println!("Yay!!! Player {} wins!!! 🎉🥳", player_name);

            let continue_answer = Game::ask("Play again? (y/n): ");

            match continue_answer.as_str() {
                "y" => {
                    self.reset();
                    return;
                }
                _ => {
                    self.state = GameState::WillExit;
                    return;
                }
            }
        }

        let input = Game::ask(
            format!(
                "> Player {} ({})'s turn - enter x and y: ",
                player_name, next_tile_state
            )
            .as_str(),
        );

        match input.as_str() {
            "q" => return,
            "exit" => return,
            "y/n" => {}
            _ => match input.parse::<play::Play>() {
                Ok(play) => match self.board.update(play.x, play.y, next_tile_state) {
                    Ok(_) => {
                        if self.board.any_row_won() {
                            self.state = GameState::Won;
                            return;
                        }

                        println!("Player {}'s turn!", player_name);

                        self.move_count += 1;
                    }
                    Err(e) => match e {
                        BoardUpdateError::AlreadyOccupied => println!("Can't move there!"),
                        BoardUpdateError::TileNonExistent => {
                            println!("Enter x and y coords between 1 and 3!")
                        }
                    },
                },
                Err(e) => match e {
                    play::ParsePlayError::BadLen => {
                        println!("Please enter x and y coords like: 1 2")
                    }
                    play::ParsePlayError::ParseInt(e) => {
                        println!("Can't convert {} to an integer!", e)
                    }
                },
            },
        }
    }

    fn reset(&mut self) {
        self.move_count = 0;
        self.board.reset();
        self.state = GameState::Running;
    }

    fn ask(message: &str) -> String {
        if !message.is_empty() {
            print!("{}", message);
            io::stdout().flush().unwrap();
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(error) => {
                println!("{:?}", error);
                return String::new();
            }
            _ => (),
        };

        input.trim().to_string()
    }
}
