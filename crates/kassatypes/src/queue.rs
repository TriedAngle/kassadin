#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum QueueType {
    RankedSolo5x5,
    RankedFlexSr,
    RankedFlexTT,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GameType {
    CustomGame,
    MatchedGame,
    TutorialGame,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GameMode {
    Aram,
    ARSR,
    Ascension,
    Assassinate,
    Classic,
    DarkStar,
    DoomBotsTeemo,
    FirstBlood,
    GameModeX,
}
