use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Object{
    Baba,
    Flag,
    Rock,
    Wall,
    Key,
}