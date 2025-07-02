mod board;
mod play;
mod game;

fn main() {
    let mut game_instance = game::Game::default();
    
    game_instance.run()
}


