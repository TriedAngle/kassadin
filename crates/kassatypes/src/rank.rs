#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Division {
    I = 3,
    II = 2,
    III = 1,
    IV = 0,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tier {
    Challenger = 9,
    Grandmaster = 8,
    Master = 7,
    Diamond = 6,
    Platinum = 5,
    Gold = 4,
    Silver = 3,
    Bronze = 2,
    Iron = 1,
    Unranked = 0,
}

impl Tier {
    pub const fn is_apex(self) -> bool {
        // self  > Self::Master doesn't work in const contexts ?XD
        self as u8 > Self::Master as u8
    }

    pub const fn is_ranked(self) -> bool {
        // self  != Self::Unranked doesn't work in const contexts ?XD
        self as u8 != Self::Unranked as u8
    }
}