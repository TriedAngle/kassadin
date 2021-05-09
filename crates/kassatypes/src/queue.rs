#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
pub enum Queue {
    #[cfg_attr(feature = "serde", serde(rename = "RANKED_SOLO_5x5"))]
    #[strum(to_string = "RANKED_SOLO_5x5")]
    RankedSolo5x5,
    #[cfg_attr(feature = "serde", serde(rename = "RANKED_FLEX_SR"))]
    #[strum(to_string = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[cfg_attr(feature = "serde", serde(rename = "RANKED_FLEX_TT"))]
    #[strum(to_string = "RANKED_FLEX_TT")]
    RankedFlexTT,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
pub enum GameType {
    CustomGame,
    MatchedGame,
    TutorialGame,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(
    EnumString,
    EnumIter,
    Display,
    AsRefStr,
    IntoStaticStr,
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
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
