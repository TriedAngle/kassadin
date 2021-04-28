#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Region {
    #[cfg_attr(feature = "serde", serde(rename = "BR1"))]
    BR,
    #[cfg_attr(feature = "serde", serde(rename = "EUN1"))]
    EUNE,
    #[cfg_attr(feature = "serde", serde(rename = "EUW1"))]
    EUW,
    #[cfg_attr(feature = "serde", serde(rename = "NA1"))]
    NA,
    KR,
    #[cfg_attr(feature = "serde", serde(rename = "LA1"))]
    LAN,
    #[cfg_attr(feature = "serde", serde(rename = "LA2"))]
    LAS,
    #[cfg_attr(feature = "serde", serde(rename = "OC1"))]
    OCE,
    RU,
    #[cfg_attr(feature = "serde", serde(rename = "TR1"))]
    TR,
    #[cfg_attr(feature = "serde", serde(rename = "JP1"))]
    JP,
    #[cfg_attr(feature = "serde", serde(rename = "PBE1"))]
    PBE,
    AMERICAS,
    EUROPE,
    ASIA,
}

impl Region {
    pub fn as_str(&self) -> &str {
        match self {
            Region::BR => "BR1",
            Region::EUNE => "EUN1",
            Region::EUW => "EUW1",
            Region::NA => "NA1",
            Region::KR => "KR",
            Region::LAN => "LA1",
            Region::LAS => "LA2",
            Region::OCE => "OC1",
            Region::RU => "RU",
            Region::TR => "TR1",
            Region::JP => "JP1",
            Region::PBE => "PBE1",
            Region::AMERICAS => "AMERICAS",
            Region::EUROPE => "EUROPE",
            Region::ASIA => "ASIA",
        }
    }
}

impl Into<String> for Region {
    fn into(self) -> String {
        String::from(self.as_str())
    }
}

impl ToString for Region {
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}