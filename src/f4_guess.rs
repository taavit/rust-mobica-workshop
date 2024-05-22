use crate::game::{Game, GameResult};

pub struct MemoryGame;

impl MemoryGame {
    pub fn new() -> Self {
        MemoryGame
    }
}

impl Game for MemoryGame {
    fn advance(&mut self) -> u8 {
        unimplemented!();
    }
    fn new_game(&mut self) {
        unimplemented!();
    }
    fn play(&mut self) -> GameResult {
        unimplemented!();
    }
}