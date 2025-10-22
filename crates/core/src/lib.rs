
use chrono::{DateTime, Local};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zone {
    A, B, C, D,
    E1, E2, E3, E4, E5, E6, E7, E8, E9,
}

impl Zone {
    pub const ALL: [Zone; 13] = [
        Zone::A, Zone::B, Zone::C, Zone::D,
        Zone::E1, Zone::E2, Zone::E3, Zone::E4, Zone::E5, Zone::E6, Zone::E7, Zone::E8, Zone::E9
    ];
}

impl Display for Zone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Zone::A => write!(f, "A"),
            Zone::B => write!(f, "B"),
            Zone::C => write!(f, "C"),
            Zone::D => write!(f, "D"),
            Zone::E1 => write!(f, "E1"),
            Zone::E2 => write!(f, "E2"),
            Zone::E3 => write!(f, "E3"),
            Zone::E4 => write!(f, "E4"),
            Zone::E5 => write!(f, "E5"),
            Zone::E6 => write!(f, "E6"),
            Zone::E7 => write!(f, "E7"),
            Zone::E8 => write!(f, "E8"),
            Zone::E9 => write!(f, "E9"),
        }
    }
}

impl TryFrom<&str> for Zone {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_ascii_uppercase().as_str() {
            "A" => Ok(Zone::A),
            "B" => Ok(Zone::B),
            "C" => Ok(Zone::C),
            "D" => Ok(Zone::D),
            "E1" => Ok(Zone::E1),
            "E2" => Ok(Zone::E2),
            "E3" => Ok(Zone::E3),
            "E4" => Ok(Zone::E4),
            "E5" => Ok(Zone::E5),
            "E6" => Ok(Zone::E6),
            "E7" => Ok(Zone::E7),
            "E8" => Ok(Zone::E8),
            "E9" => Ok(Zone::E9),
            _ => Err(format!("unknown zone: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PackageStatus {
    In,
    Out,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub id: i64,
    pub tracking: String,
    pub zone: Zone,
    pub status: PackageStatus,
    pub created_at: DateTime<Local>,
}

pub fn normalize_tracking(input: &str) -> String {
    let re = Regex::new(r"[^0-9A-Za-z]+").unwrap();
    let s = re.replace_all(input.trim(), "");
    s.to_ascii_uppercase()
}
