use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Noun{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}