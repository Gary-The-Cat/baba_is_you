use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Property{
    You,
    Push,
    Stop,
    Win,
    Sink,
    Defeat,
    Hot,
    Melt,
    Tele,
    Open,
    Shut,
}