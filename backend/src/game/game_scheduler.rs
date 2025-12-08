use std::collections::HashSet;
use crate::game::game::Game;
use crate::user::user::User;

/// 游戏调度器
#[derive(Debug)]
pub struct GamesScheduler {
    game_set: HashSet<Game>,
    participant_set: HashSet<User>
}

impl GamesScheduler {
    //控制为单例模式
    pub(self) fn new(game_set: HashSet<Game>, participant_set: HashSet<User>) -> GamesScheduler {
        GamesScheduler{game_set, participant_set}
    }

    pub fn get_game_set(&self) -> &HashSet<Game> {
        &self.game_set
    }

    pub fn get_participant_set(&self) -> &HashSet<User> {
        &self.participant_set
    }

    pub fn add_game(&mut self, game: Game) {
        self.game_set.insert(game);
    }

    pub fn add_participant(&mut self, user: User) {
        self.participant_set.insert(user);
    }

    pub fn remove_game(&mut self, game: &Game) {
        self.game_set.remove(game);
    }

    pub fn remove_participant(&mut self, user: &User) {
        self.participant_set.remove(user);
    }
}