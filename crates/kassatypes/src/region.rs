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
pub enum Region {
    #[cfg_attr(feature = "serde", serde(rename = "BR1"))]
    #[strum(to_string = "BR1")]
    BR,
    #[cfg_attr(feature = "serde", serde(rename = "EUN1"))]
    #[strum(to_string = "EUN1")]
    EUNE,
    #[cfg_attr(feature = "serde", serde(rename = "EUW1"))]
    #[strum(to_string = "EUW1")]
    EUW,
    #[cfg_attr(feature = "serde", serde(rename = "NA1"))]
    #[strum(to_string = "NA1")]
    NA,
    KR,
    #[cfg_attr(feature = "serde", serde(rename = "LA1"))]
    #[strum(to_string = "LA1")]
    LAN,
    #[cfg_attr(feature = "serde", serde(rename = "LA2"))]
    #[strum(to_string = "LA2")]
    LAS,
    #[cfg_attr(feature = "serde", serde(rename = "OC1"))]
    #[strum(to_string = "OC1")]
    OCE,
    RU,
    #[cfg_attr(feature = "serde", serde(rename = "TR1"))]
    #[strum(to_string = "TR1")]
    TR,
    #[cfg_attr(feature = "serde", serde(rename = "JP1"))]
    #[strum(to_string = "JP1")]
    JP,
    #[cfg_attr(feature = "serde", serde(rename = "PBE1"))]
    #[strum(to_string = "PBE1")]
    PBE,
    AMERICAS,
    EUROPE,
    ASIA,
    SEA
}
