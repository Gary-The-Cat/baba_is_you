use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Noun{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}