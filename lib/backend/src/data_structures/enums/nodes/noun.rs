use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Noun{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}