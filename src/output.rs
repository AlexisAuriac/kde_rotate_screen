use anyhow::{Result, anyhow};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Orientation {
    Normal = 1,
    Left = 2,
    Inverted = 4,
    Right = 8,
}

impl Orientation {
    pub fn try_from_u8(n: u8) -> Result<Self> {
        match n {
            _ if n == Orientation::Normal as u8 => Ok(Orientation::Normal),
            _ if n == Orientation::Left as u8 => Ok(Orientation::Left),
            _ if n == Orientation::Inverted as u8 => Ok(Orientation::Inverted),
            _ if n == Orientation::Right as u8 => Ok(Orientation::Right),
            _ => Err(anyhow!("invalid orientation number: {n}")),
        }
    }

    pub fn try_from_str(s: &str) -> Result<Self> {
        match s {
            "normal" => Ok(Orientation::Normal),
            "left" => Ok(Orientation::Left),
            "inverted" => Ok(Orientation::Inverted),
            "right" => Ok(Orientation::Right),
            _ => Err(anyhow!("invalid orientation string: {s}")),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug)]
pub struct Output {
    pub name: String,
    pub orientation: Orientation,
}
