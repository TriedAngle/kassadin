#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
pub enum Champion {
    Aatrox = 266,
}

impl Champion {
    pub fn as_str(&self) -> &str {
        match self {
            Champion::Aatrox => "Aatrox",
        }
    }

    pub fn id(self) -> i32 {
        self as i32
    }
}