#[derive(Debug, Hash, PartialEq, Eq)]
pub struct User{
    id: i32,
    name: String,
    balance: i32,
}

impl User{
    pub fn new(id: i32, name:String, balance: i32) -> User{
        User{id, name, balance}
    }
}