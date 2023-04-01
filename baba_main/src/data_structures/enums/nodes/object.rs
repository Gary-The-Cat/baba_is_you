use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Object{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}