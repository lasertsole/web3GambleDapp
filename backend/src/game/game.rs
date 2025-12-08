use std::fmt::Debug;
use crate::game::game_item::GameItem;
use std::collections::HashSet;
use std::hash::Hash;
use std::any::Any;
use std::collections::HashMap;
use crate::game::player::Player;
use crate::game::game_rule::GameRule;



///游戏状态
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState{
    NotStarted,
    InProgress,
    Paused,
    Finished
}


/// 游戏对局
#[derive(Debug)]
pub struct Game {
    current_players : Vec<&'static Player>,
    game_item: Vec<&'static dyn GameItem>,
    game_rule: &'static mut GameRule,
    game_state: GameState,
    game_context: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl Game {
    pub fn new(
        current_players: Vec<&'static Player>,
        game_item: Vec<&'static dyn GameItem>,
        game_rule: &'static mut GameRule,
        game_state: GameState,
        game_context: HashMap<String, Box<dyn Any + Send + Sync>>,
    ) -> Game {
        Game {
            current_players,
            game_item,
            game_rule,
            game_state,
            game_context
        }
    }

    pub fn player_join(&mut self, join_players: &Vec<&'static Player>) -> () {
        self.current_players.extend(join_players);
        (self.game_rule.players_join) (
            join_players,
            &self.current_players,
            &self.game_item,
            self.game_state,
            &self.game_context
        );
    }

    pub fn player_leave(&mut self, leave_players: &Vec<&'static Player>) -> (){
        let leave_players_set: HashSet<_> = leave_players.into_iter().collect();
        self.current_players.retain(|&element| !leave_players_set.contains(&element));
        (self.game_rule.players_leave) (
            leave_players,
            &self.current_players,
            &self.game_item,
            self.game_state,
            &self.game_context
        );
    }

    pub fn game_start(&mut self) -> () {
        assert_eq!(self.game_state, GameState::NotStarted, "game state isn't NotStarted");

        self.game_state = GameState::InProgress;
        (self.game_rule.game_start) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }

    pub fn game_pause(&mut self) -> () {
        assert_eq!(self.game_state, GameState::InProgress, "game state isn't InProgress");

        self.game_state = GameState::Paused;
        (self.game_rule.game_start) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }

    pub fn game_resume(&mut self) -> () {
        assert_eq!(self.game_state, GameState::Paused, "game state isn't Paused");

        self.game_state = GameState::InProgress;
        (self.game_rule.game_resume) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }

    pub fn game_progress(&mut self) -> () {
        assert_eq!(self.game_state, GameState::InProgress, "game state isn't InProgress");

        (self.game_rule.game_progress) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }

    pub fn game_finish(&mut self) -> () {
        assert_eq!(self.game_state, GameState::InProgress, "game state isn't InProgress");

        self.game_state = GameState::Finished;
        (self.game_rule.game_finish) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }

    pub fn game_wait_start(&mut self) -> () {
        assert_eq!(self.game_state, GameState::Finished, "game state isn't Finished");

        self.game_state = GameState::NotStarted;
        (self.game_rule.game_wait_start) (
            &self.current_players,
            &self.game_item,
            &self.game_context
        );
    }
}

// 分别实现 Hash、PartialEq、Eq的trait，使dyn GameRule可比较哈希值，从而可以插入HashSet
impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 哈希参与者
        self.current_players.hash(state);
        // 哈希 game_rule 指针的地址
        std::ptr::hash(self.game_rule as *const _, state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        // 比较参与者列表
        self.current_players == other.current_players &&
            // 比较 game_rule 指针的地址，以判断是否是同一个实例
            std::ptr::eq(self.game_rule as *const _, other.game_rule as *const _)
    }
}

impl Eq for Game {}
