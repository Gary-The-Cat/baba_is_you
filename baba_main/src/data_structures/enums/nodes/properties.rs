use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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