use std::any::Any;
use std::fmt::Debug;

/// 游戏物品trait
pub trait GameItem: Debug + Any + Send + Sync {}