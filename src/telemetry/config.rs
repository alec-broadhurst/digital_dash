pub enum Game {
    Forza,
    None,
}

impl Game {
    pub fn detect_game() -> Game {
        Game::Forza
    }
}
