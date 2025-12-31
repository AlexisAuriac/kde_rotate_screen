use std::str::FromStr;

use anyhow::{Result, anyhow};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn to_str(self) -> &'static str {
        match self {
            Orientation::Normal => "normal",
            Orientation::Left => "left",
            Orientation::Inverted => "inverted",
            Orientation::Right => "right",
        }
    }

    pub fn rotate_clockwise(self) -> Self {
        match self {
            Orientation::Normal => Orientation::Right,
            Orientation::Left => Orientation::Normal,
            Orientation::Inverted => Orientation::Left,
            Orientation::Right => Orientation::Inverted,
        }
    }

    pub fn rotate_counter_clockwise(self) -> Self {
        match self {
            Orientation::Normal => Orientation::Left,
            Orientation::Left => Orientation::Inverted,
            Orientation::Inverted => Orientation::Right,
            Orientation::Right => Orientation::Normal,
        }
    }
}

impl FromStr for Orientation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Orientation::Normal),
            "left" => Ok(Orientation::Left),
            "inverted" => Ok(Orientation::Inverted),
            "right" => Ok(Orientation::Right),
            _ => Err(anyhow!("invalid orientation string: {s}")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub orientation: Orientation,
    pub enabled: bool,
    pub current_mode: String,
}
