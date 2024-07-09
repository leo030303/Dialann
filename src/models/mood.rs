use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Mood {
    Awesome,
    Good,
    Okay,
    Bad,
    Awful,
}

impl Mood {
    pub fn from_int(int: i8) -> Self {
        match int {
            1 => Mood::Awful,
            2 => Mood::Bad,
            3 => Mood::Okay,
            4 => Mood::Good,
            5 => Mood::Awesome,
            _ => Mood::Awful,
        }
    }

    pub fn to_int(&self) -> i8 {
        match self {
            Mood::Awful => 1,
            Mood::Bad => 2,
            Mood::Okay => 3,
            Mood::Good => 4,
            Mood::Awesome => 5,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Mood::Awesome => "Awesome",
            Mood::Good => "Good",
            Mood::Okay => "Okay",
            Mood::Bad => "Bad",
            Mood::Awful => "Awful",
        }
    }
    pub fn icon_url(&self) -> &'static str {
        match self {
            Mood::Awesome => "public/awesome.svg",
            Mood::Good => "public/good.svg",
            Mood::Okay => "public/okay.svg",
            Mood::Bad => "public/bad.svg",
            Mood::Awful => "public/awful.svg",
        }
    }
    pub fn colour_code(&self) -> &'static str {
        match self {
            Mood::Awesome => "#34b296",
            Mood::Good => "#78be32",
            Mood::Okay => "#f8db01",
            Mood::Bad => "#f68a23",
            Mood::Awful => "#d9170e",
        }
    }
}
