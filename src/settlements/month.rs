use std::fmt::{Display, Formatter};
use std::str::FromStr;

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Sequence)]
#[repr(u8)]
pub enum Month {
    JAN = 1,
    FEB = 2,
    MAR = 3,
    APR = 4,
    MAY = 5,
    JUN = 6,
    JUL = 7,
    AUG = 8,
    SEP = 9,
    OCT = 10,
    NOV = 11,
    DEC = 12,
}

impl FromStr for Month {
    type Err = String;

    fn from_str(month: &str) -> Result<Self, Self::Err> {
        match month.trim().to_uppercase().as_ref() {
            "JAN" => Ok(Month::JAN),
            "FEB" => Ok(Month::FEB),
            "MAR" => Ok(Month::MAR),
            "APR" => Ok(Month::APR),
            "MAY" => Ok(Month::MAY),
            "JUN" => Ok(Month::JUN),
            "JUL" => Ok(Month::JUL),
            "AUG" => Ok(Month::AUG),
            "SEP" => Ok(Month::SEP),
            "OCT" => Ok(Month::OCT),
            "NOV" => Ok(Month::NOV),
            "DEC" => Ok(Month::DEC),
            _ => Err(format!("'{}' is not a valid month!", month)),
        }
    }
}

impl Display for Month {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Month::JAN => write!(formatter, "JAN"),
            Month::FEB => write!(formatter, "FEB"),
            Month::MAR => write!(formatter, "MAR"),
            Month::APR => write!(formatter, "APR"),
            Month::MAY => write!(formatter, "MAY"),
            Month::JUN => write!(formatter, "JUN"),
            Month::JUL => write!(formatter, "JUL"),
            Month::AUG => write!(formatter, "AUG"),
            Month::SEP => write!(formatter, "SEP"),
            Month::OCT => write!(formatter, "OCT"),
            Month::NOV => write!(formatter, "NOV"),
            Month::DEC => write!(formatter, "DEC"),
        }
    }
}
