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
    Tied,
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
        self.render();

        loop {
            match self.state {
                GameState::WillExit => break,
                GameState::Running => {
                    self.update();
                    self.render();
                }
                GameState::Won => self.on_win(),
                GameState::Tied => self.on_tie(),
            }
        }
    }

    pub fn update(&mut self) {
        let player_name = self.get_player(self.player_id());
        let next_tile_state = self.calc_next_tile_state();
        let input = Game::ask(
            format!(
                "Player {} ({})'s turn - enter x and y: ",
                player_name, next_tile_state
            )
            .as_str(),
        );

        match input.as_str() {
            "q" => self.state = GameState::WillExit,
            "exit" => self.state = GameState::WillExit,
            _ => self.on_move_input(&input),
        }
    }

    pub fn render(&self) {
        println!("{}", self.board);
    }

    fn reset(&mut self) {
        self.move_count = 0;
        self.board.reset();
        self.state = GameState::Running;
    }

    fn ask(message: &str) -> String {
        if !message.is_empty() {
            print!("> {}", message);
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

    fn confirm(message: &str) -> bool {
        let resp = Self::ask(message);

        match resp.as_str() {
            "y" => true,
            "yes" => true,
            _ => false,
        }
    }

    fn on_move_input(&mut self, input: &String) {
        match input.parse::<play::Play>() {
            Ok(play) => match self
                .board
                .update(play.x, play.y, self.calc_next_tile_state())
            {
                Ok(_) => {
                    if self.board.any_row_won() {
                        self.state = GameState::Won;
                        return;
                    }

                    if self.board.full() {
                        self.state = GameState::Tied;
                        return;
                    }

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
                play::ParsePlayError::ParseInt => {
                    println!("ENTER A NUMBER PLEASE. '{}' ARE NOT NUMBERS!", input)
                }
            },
        };
    }

    fn on_win(&mut self) {
        let player_id = self.get_player(self.player_id());

        self.on_end(format!("Yay!!! Player {} wins!!! 🎉🥳", player_id))
    }

    fn on_tie(&mut self) {
        self.on_end("Womp womp womp! No one wins!".into())
    }

    fn on_end(&mut self, message: String) {
        println!("{}", message);

        if Self::confirm("Play again? (y/n): ") {
            self.reset()
        } else {
            self.state = GameState::WillExit;
        }
    }

    fn get_player(&mut self, player_id: u32) -> String {
        if !self.players.contains_key(&player_id) {
            let name = Game::ask(format!("Player {}, what's your name?: ", player_id).as_str());
            self.players.insert(player_id, name);
        }

        self.players.entry(player_id).or_default().to_string()
    }

    fn player_id(&self) -> u32 {
        self.move_count % 2 + 1
    }

    fn calc_next_tile_state(&self) -> board::TileState {
        match self.move_count % 2 {
            0 => board::TileState::Ex,
            _ => board::TileState::Oh,
        }
    }
}
