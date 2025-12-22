use std::collections::HashMap;
use std::hash::Hash;
use crate::game::game_projects::game_project::GameProject;
use crate::game::player::Player;

#[derive(Debug)]
pub struct User{
    id: u32,
    name: String,
    balance: u32,
    cur_player_map: HashMap<GameProject, Player>,
    token_count_map: HashMap<u32, GameProject>,
}

impl User{
    pub fn new(id: u32, name:String, balance: u32) -> User{
        User{id, name, balance, cur_player_map: HashMap::new(), token_count_map: HashMap::new()}
    }
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        // 比较参与者列表
        self.id == other.id
    }
}

impl Eq for User {}