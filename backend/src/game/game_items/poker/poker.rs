// 引入标准库的 Vec
use std::vec::Vec;
use std::fmt::Debug;
use crate::game::game_item::GameItem;

// 扑克花色
#[derive(Debug, Clone, Copy)] // 添加 derive 宏以方便复制和调试
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds
}

// 为 Suit 提供一个包含所有花色的常量数组
impl Suit {
    pub const ALL_SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
}

// 扑克点数
#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Ace,   // A
    Two,   // 2
    Three, // 3
    Four,  // 4
    Five,  // 5
    Six,   // 6
    Seven, // 7
    Eight, // 8
    Nine,  // 9
    Ten,   // 10
    Jack,  // J
    Queen, // Q
    King,  // K
}

// 为 Rank 提供一个包含所有点数的常量数组
impl Rank {
    pub const ALL_RANKS: [Rank; 13] = [
        Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six,
        Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King
    ];
}

// 扑克卡对象
#[derive(Debug, Clone)]
pub struct Card{
    pub suit: Suit,
    pub rank: Rank,
}

impl GameItem for Card{}

impl Card{
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card{suit, rank}
    }
}

// 获取完整的52张牌组
pub fn get_all_cards() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);

    // 遍历所有花色
    for suit in Suit::ALL_SUITS {
        // 遍历所有点数
        for rank in Rank::ALL_RANKS {
            // 为每种花色和点数组合创建一张新牌
            deck.push(Card::new(suit, rank));
        }
    }

    deck
}