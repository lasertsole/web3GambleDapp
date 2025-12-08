#[macro_use]
extern crate lazy_static;
mod user;
use game::game_items::poker::poker::get_all_cards;
mod game;
mod timer;
mod event;

fn main() {
    println!("{:?} ", get_all_cards());
    // println!("{:?} ", *game::game::GLOBAL_GAMES_SCHEDULER);
}
