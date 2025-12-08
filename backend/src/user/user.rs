use std::collections::HashMap;
use std::hash::Hash;
use crate::game::game_projects::game_project::GameProject;
use crate::game::player::Player;

#[derive(Debug)]
pub struct User{
    id: i32,
    name: String,
    balance: i32,
    cur_player_map: HashMap<GameProject, Player>,
}

impl User{
    pub fn new(id: i32, name:String, balance: i32) -> User{
        User{id, name, balance, cur_player_map: HashMap::new()}
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