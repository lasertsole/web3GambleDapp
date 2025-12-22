use crate::game::game_projects::game_project::GameProject;

#[derive(Debug)]
pub struct GameTokens {
    target_game: GameProject,
    price_from_balance: u8,
}

impl GameTokens {
    pub fn new(target_game: GameProject, price_from_balance: u8) -> Self {
        GameTokens(target_game, price_from_balance)
    }
}