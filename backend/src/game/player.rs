use std::hash::Hash;
use crate::game::game_item::GameItem;
use crate::user::user::User;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlayerRole{
    Player,// 玩家
    Dealer// 荷官
}

/// 玩家
#[derive(Debug)]
pub struct Player {
    player_role: PlayerRole,
    user: &'static User,
    game_item: &'static Vec<&'static dyn GameItem>,
    chip: u16,// 筹码数量
}

impl Player {
    pub fn new(player_role: PlayerRole, user: &'static User, game_item: &'static Vec<&'static dyn GameItem>, chip: u16) -> Self{
        Player {player_role, user, game_item, chip}
    }

    pub fn update_player_role(&mut self, new_role: PlayerRole){
        self.player_role = new_role;
    }

    pub fn update_game_item(&mut self, new_game_item: &'static Vec<&'static dyn GameItem>){
        self.game_item = new_game_item;
    }
}

// 分别实现 Hash、PartialEq、Eq的trait，使dyn GameItem可比较哈希值，从而可以插入HashSet
impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 哈希参与者
        self.user.hash(state);
        // 哈希 game_rule 指针的地址
        std::ptr::hash(self.game_item as *const _, state);
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        // 比较参与者列表
        self.user == other.user &&
            // 比较 game_rule 指针的地址，以判断是否是同一个实例
            std::ptr::eq(self.game_item as *const _, other.game_item as *const _)
    }
}

impl Eq for Player {}