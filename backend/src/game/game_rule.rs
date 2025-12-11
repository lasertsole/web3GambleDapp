use std::fmt;
use std::any::Any;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::game::game_item::GameItem;
use crate::game::game::GameState;
use crate::game::player::Player;
use crate::timer::timer::CBTimesMethod;


pub type CompareCB = Arc<dyn Fn(
    &Vec<&dyn GameItem>,
    &Vec<&dyn GameItem>,
    Arc<HashMap<String, Arc<dyn Any + Send + Sync>>>
) -> bool>;

pub type GameCB = Arc<dyn Fn(
    Arc<Mutex<Vec<Arc<Player>>>>,
    Arc<Mutex<Vec<Arc<dyn GameItem>>>>,
    Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>
) -> ()>;

pub type PlayersCB = Arc<dyn Fn(
    Arc<Mutex<Vec<Arc<Player>>>>,
    Arc<Mutex<Vec<Arc<Player>>>>,
    Arc<Mutex<Vec<Arc<dyn GameItem>>>>, Arc<Mutex<GameState>>, Arc<Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>>
)->()>;

/// 游戏规则错误类型
pub enum GameRuleError {
    TimerConfigMismatch,
}

/// 游戏规则
pub struct GameRule {
    pub compare: CompareCB,
    pub allocate: GameCB,
    pub game_start: GameCB,
    pub game_progress: GameCB,
    pub game_pause: GameCB,
    pub game_resume: GameCB,
    pub game_finish: GameCB,
    pub game_wait_start: GameCB,
    pub game_timeout: GameCB,
    pub game_timer_duration: Option<Duration>,
    pub game_timer_times_method: Option<CBTimesMethod>,
    pub players_join: PlayersCB,
    pub players_leave: PlayersCB,
    pub players_timeout: PlayersCB,
    pub players_timer_duration: Option<Duration>,
    pub players_timer_times_method: Option<CBTimesMethod>,
}

impl GameRule {
    pub fn new(
        compare: CompareCB,
        allocate: GameCB,
        game_start: GameCB,
        game_progress: GameCB,
        game_pause: GameCB,
        game_resume: GameCB,
        game_finish: GameCB,
        game_wait_start: GameCB,
        game_timeout: GameCB,
        game_timer_duration: Option<Duration>,
        game_timer_times_method: Option<CBTimesMethod>,
        players_join: PlayersCB,
        players_leave: PlayersCB,
        players_timeout: PlayersCB,
        players_timer_duration: Option<Duration>,
        players_timer_times_method: Option<CBTimesMethod>,
    ) -> Result<Self, GameRuleError> {
        // game_timer前缀属性必须同时为空或同时不为空
        if game_timer_duration.is_some() != game_timer_times_method.is_some() {
            return Err(GameRuleError::TimerConfigMismatch);
        }
        // players_timer前缀属性必须同时为空或同时不为空
        if players_timer_duration.is_some() != players_timer_times_method.is_some() {
            return Err(GameRuleError::TimerConfigMismatch);
        }

        Ok(GameRule {
            compare,
            allocate,
            game_start,
            game_progress,
            game_pause,
            game_resume,
            game_finish,
            game_wait_start,
            game_timeout,
            game_timer_duration,
            game_timer_times_method,
            players_join,
            players_leave,
            players_timeout,
            players_timer_duration,
            players_timer_times_method,
        })
    }
}

// 手动实现 Debug Trait
impl fmt::Debug for GameRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameRule")
            .field("compare", &"Arc[Fn compare]")
            .field("allocate", &"Arc[Fn allocate]")
            .field("game_start", &"Arc[Fn game_start]")
            .field("game_progress", &"Arc[Fn game_progress]")
            .field("game_pause", &"Arc[Fn game_pause]")
            .field("game_resume", &"Arc[Fn game_resume]")
            .field("game_finish", &"Arc[Fn game_finish]")
            .field("game_wait_start", &"Arc[Fn game_wait_start]")
            .field("game_timeout", &"Arc[Fn game_timeout]")
            .field("game_timer_duration", &self.game_timer_duration)
            .field("game_timer_times_method", &self.game_timer_times_method)
            .field("players_join", &"Arc[Fn players_join]")
            .field("players_leave", &"Arc[Fn players_leave]")
            .field("players_timeout", &"Arc[Fn players_timeout]")
            .field("players_timer_duration", &self.players_timer_duration)
            .field("players_timer_times_method", &self.players_timer_times_method)
            .finish()
    }
}
