use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Object{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}