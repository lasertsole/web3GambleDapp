use std::fmt::{write, Debug};
use crate::game::game_item::GameItem;
use std::collections::HashSet;
use std::hash::Hash;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::game::player::Player;
use crate::game::game_rule::{GameCB, GameRule, PlayersCB};
use crate::timer::timer::Timer;

///游戏状态
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState{
    NotStarted,
    InProgress,
    Paused,
    Finished
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameState::NotStarted => write!(f, "NotStarted"),
            GameState::InProgress => write!(f, "InProgress"),
            GameState::Paused => write!(f, "Paused"),
            GameState::Finished=>write!(f, "Finished"),
        }
    }
}

struct Tuple(
    pub Arc<Mutex<Vec<Arc<Player>>>>,
    pub Arc<Mutex<Vec<Arc<Player>>>>,
    pub Arc<Mutex<Vec<Arc<dyn GameItem>>>>,
    pub Arc<Mutex<GameState>>,
    pub Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    pub GameCB,
    pub PlayersCB,
);

impl fmt::Debug for Tuple{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tuple")
        .field("0", &self.0)
        .field("1", &self.1)
        .field("2", &self.2)
        .field("3", &self.3)
        .field("4", &self.3)
        .field("5", &"Arc[Fn]")
        .field("6", &"Arc[Fn]")
        .finish() // 结束构建并返回 Result
    }
}

/// 游戏对局
#[derive(Debug)]
pub struct Game {
    current_players : Arc<Mutex<Vec<Arc<Player>>>>,
    current_action_players : Arc<Mutex<Vec<Arc<Player>>>>,
    game_item: Arc<Mutex<Vec<Arc<dyn GameItem>>>>,
    game_rule: &'static mut GameRule,
    game_state: Arc<Mutex<GameState>>,
    game_context: Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>,
    game_timer_for_whole: Mutex<Option<Timer<Tuple>>>,
    game_timer_for_players: Mutex<Option<Timer<Tuple>>>,
}

impl Game {
    pub fn new(
        game_item: Arc<Mutex<Vec<Arc<dyn GameItem>>>>,
        game_rule: &'static mut GameRule,
    ) -> Self {
        Game {
            current_players:Arc::new(Mutex::new(Vec::new())),
            current_action_players:Arc::new(Mutex::new(Vec::new())),
            game_item,
            game_rule,
            game_state: Arc::new(Mutex::new(GameState::NotStarted)),
            game_context:Arc::new(Mutex::new(HashMap::new())),
            game_timer_for_whole: Mutex::new(None),
            game_timer_for_players: Mutex::new(None),
        }
    }

    fn init(&mut self) {
        let tuple:Arc<Tuple> = Arc::new(
            Tuple(
                self.current_players.clone(),
                self.current_players.clone(),
                self.game_item.clone(),
                self.game_state.clone(),
                self.game_context.clone(),
                self.game_rule.game_timeout.clone(),
                self.game_rule.players_timeout.clone()
            )
        );
        let tuple_clone = tuple.clone();

        if let Some(cb_times_method) = self.game_rule.game_timer_times_method{
            let game_timeout:Box<dyn FnMut(Option<Arc<Tuple>>)->()>
                = Box::new(|option_tuple: Option<Arc<Tuple>>| {
                if let Some(tuple) = option_tuple {
                    let players_clone = tuple.0.clone();
                    let game_items_clone = tuple.2.clone();
                    let context_clone = tuple.4.clone();
                    let game_timeout = tuple.5.clone();
                    (game_timeout)(players_clone, game_items_clone, context_clone)
                }
            });

            self.set_game_timer_for_whole(Mutex::new(
                Some(Timer::<Tuple>::new(
                    self.game_rule.game_timer_duration,
                    Some(tuple),
                    game_timeout,
                    cb_times_method,
                ))
            ));
        }

        if let Some(cb_times_method) = self.game_rule.players_timer_times_method{

            let player_timeout:Box<dyn FnMut(Option<Arc<Tuple>>)->()>
                = Box::new(|option_tuple: Option<Arc<Tuple>>| {
                if let Some(tuple) = option_tuple {
                    let players_clone = tuple.0.clone();
                    let action_players_clone = tuple.1.clone();
                    let game_items_clone = tuple.2.clone();
                    let game_state = tuple.3.clone();
                    let context_clone = tuple.4.clone();
                    let player_timeout = tuple.6.clone();
                    (player_timeout)(players_clone, action_players_clone, game_items_clone, game_state, context_clone)
                }
            });

            self.set_game_timer_for_players(
                Mutex::new(
                    Some(Timer::new(
                        self.game_rule.game_timer_duration,
                        Some(tuple_clone),
                        Box::new(player_timeout),
                        cb_times_method,
                    ))
                )
            );
        }
    }

    fn set_game_timer_for_whole(&mut self, option_timer:Mutex<Option<Timer<Tuple>>>)->(){
        self.game_timer_for_whole = option_timer;
    }

    fn set_game_timer_for_players(&mut self, option_timer:Mutex<Option<Timer<Tuple>>>)->(){
        self.game_timer_for_players = option_timer;
    }

    fn set_all_timer_is_running(&mut self, is_running: bool) -> () {
        match self.game_timer_for_whole.lock() {
            Ok(mut guard) => {
                if let Some(mut item) = guard.take() {
                    item.set_is_running(is_running);
                };
            },
            Err(poisoned) => {
                print!("The lock is poisoned! Attempting to unpoison (or recover) the data and resume operations.");
                if let Some(mut item) = poisoned.into_inner().take(){
                    item.set_is_running(is_running);
                };
            }
        }

        match self.game_timer_for_players.lock() {
            Ok(mut guard) => {
                if let Some(mut item) = guard.take() {
                    item.set_is_running(is_running);
                };
            },
            Err(poisoned) => {
                print!("The lock is poisoned! Attempting to unpoison (or recover) the data and resume operations.");
                if let Some(mut item) = poisoned.into_inner().take(){
                    item.set_is_running(is_running);
                };
            }
        }
    }

    fn  translate_game_state(&self, ori_game_state:GameState, tar_game_state: Option<GameState>) -> () {
        let error_message = "game state isn't ".to_owned() + ori_game_state.to_string().as_str();

        match self.game_state.lock() {
            Ok(mut guard) => {
                assert_eq!(*guard, ori_game_state, "{}", error_message);
                if let Some(tar_game_state) = tar_game_state{
                    *guard = tar_game_state;
                }
            },
            Err(poisoned) => {
                print!("The lock is poisoned! Attempting to unpoison (or recover) the data and resume operations.");
                let mut guard = poisoned.into_inner();
                assert_eq!(*guard, ori_game_state, "{}", error_message);
                if let Some(tar_game_state) = tar_game_state{
                    *guard = tar_game_state;
                }
            },
        }
    }

    pub fn player_join(&mut self, join_players: Vec<Arc<Player>>) -> () {
        self.current_players.lock().unwrap().extend(join_players.clone());
        (self.game_rule.players_join) (
            Arc::new(Mutex::new(join_players)),
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_state.clone(),
            self.game_context.clone()
        );
    }

    pub fn player_leave(&mut self, leave_players: Vec<Arc<Player>>) -> (){
        let leave_players_set: HashSet<_> = leave_players.clone().into_iter().collect();

        match self.current_players.lock() {
            Ok(mut guard) => {
                guard.retain(|element| !leave_players_set.contains(element));
            },
            Err(poisoned) => {
                print!("The lock is poisoned! Attempting to unpoison (or recover) the data and resume operations.");
                poisoned.into_inner().retain(|element| !leave_players_set.contains(element));
            }
        }

        (self.game_rule.players_leave) (
            Arc::new(Mutex::new(leave_players)),
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_state.clone(),
            self.game_context.clone()
        );
    }

    pub fn game_start(&mut self) -> () {
        self.translate_game_state(GameState::NotStarted, Some(GameState::InProgress));

        self.set_all_timer_is_running(true);

        (self.game_rule.game_start) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone(),
        );
    }

    pub fn game_pause(&mut self) -> () {
        self.translate_game_state(GameState::InProgress, Some(GameState::Paused));

        self.set_all_timer_is_running(false);

        (self.game_rule.game_pause) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone(),
        );
    }

    pub fn game_resume(&mut self) -> () {
        self.translate_game_state(GameState::Paused, Some(GameState::InProgress));

        self.set_all_timer_is_running(true);

        (self.game_rule.game_resume) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone(),
        );
    }

    pub fn game_progress(&mut self) -> () {
        self.translate_game_state(GameState::InProgress, None);

        (self.game_rule.game_progress) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone(),
        );
    }

    pub fn game_finish(&mut self) -> () {
        self.translate_game_state(GameState::InProgress, Some(GameState::Finished));

        self.set_all_timer_is_running(false);

        (self.game_rule.game_finish) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone()
        );
    }

    pub fn game_wait_start(&mut self) -> () {
        self.translate_game_state(GameState::Finished, Some(GameState::NotStarted));

        (self.game_rule.game_wait_start) (
            self.current_players.clone(),
            self.game_item.clone(),
            self.game_context.clone()
        );
    }
}

// 分别实现 Hash、PartialEq、Eq的trait，使dyn GameRule可比较哈希值，从而可以插入HashSet
impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 哈希参与者
        self.current_players.lock().unwrap().hash(state);
        // 哈希 game_rule 指针的地址
        std::ptr::hash(self.game_rule as *const _, state);
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        // 比较参与者列表
        *self.current_players.lock().unwrap() == *other.current_players.lock().unwrap() &&
            // 比较 game_rule 指针的地址，以判断是否是同一个实例
            std::ptr::eq(self.game_rule as *const _, other.game_rule as *const _)
    }
}

impl Eq for Game {}
