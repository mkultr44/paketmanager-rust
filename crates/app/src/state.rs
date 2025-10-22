
use chrono::{DateTime, Local};
use hermes_core::{Zone};
use std::collections::HashMap;

#[derive(Default)]
pub struct Counters {
    pub per_zone: HashMap<Zone, u64>,
    pub total_in: u64,
}

pub struct AppState {
    pub current_zone: Option<Zone>;
    pub input: String,
    pub counters: Counters,
    pub last_scan: Option<(String, DateTime<Local>)>,
    pub message: Option<(String, DateTime<Local>, egui::Color32)>,
    pub debounce_ms: u64,
}

impl AppState {
    pub fn new(debounce_ms: u64) -> Self {
        Self {
            current_zone: None,
            input: String::new(),
            counters: Counters::default(),
            last_scan: None,
            message: None,
            debounce_ms,
        }
    }
}
