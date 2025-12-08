use std::any::Any;
use std::collections::HashMap;
use crate::game::game_item::GameItem;
use crate::game::game::GameState;
use crate::game::player::Player;
use std::fmt; // 引入 fmt 模块

/// 游戏规则
pub struct GameRule {
    pub compare:Box<dyn Fn(&Vec<&dyn GameItem>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> bool>,
    pub allocate:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> ()>,
    pub game_start:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub game_progress:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub game_pause:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub game_resume:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub game_finish:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub game_wait_start:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub players_join: Box<dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
    pub players_leave: Box<dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>
}

impl GameRule {
    pub fn new(
        compare: Box<dyn Fn(&Vec<&dyn GameItem>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> bool>,
        allocate: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>) -> ()>,
        game_start: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        game_progress: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        game_pause: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        game_resume: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        game_finish: Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        game_wait_start:Box<dyn Fn(&Vec<&Player>, &Vec<&dyn GameItem>, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        players_join: Box<dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>,
        players_leave: Box<dyn Fn(&Vec<&Player>, &Vec<&Player>, &Vec<&dyn GameItem>, GameState, &HashMap<String, Box<dyn Any + Send + Sync>>)->()>
    ) -> Self{
        GameRule {
            compare,
            allocate,
            game_start,
            game_progress,
            game_pause,
            game_resume,
            game_finish,
            game_wait_start,
            players_join,
            players_leave
        }
    }
}

// 手动实现 Debug Trait
impl fmt::Debug for GameRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameRule")
            .field("compare", &"Box[Fn compare]")
            .field("allocate", &"Box[Fn allocate]")
            .field("game_start", &"Box[Fn game_start]")
            .field("game_progress", &"Box[Fn game_progress]")
            .field("game_pause", &"Box[Fn game_pause]")
            .field("game_resume", &"Box[Fn game_resume]")
            .field("game_finish", &"Box[Fn game_finish]")
            .field("game_wait_start", &"Box[Fn game_wait_start]")
            .field("players_join", &"Box[Fn players_join]")
            .field("players_leave", &"Box[Fn players_leave]")
            .finish()
    }
}
