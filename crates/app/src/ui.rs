
use chrono::Local;
use egui::{RichText};
use hermes_core::{Zone};
use hermes_db::Db;
use crate::state::AppState;

pub fn ui(app: &mut eframe::AppCreatorCtx<'_>, ctx: &egui::Context, state: &mut AppState, db: &Db) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| {
            let ztxt = state.current_zone.map(|z| z.to_string()).unwrap_or_else(|| "—".to_string());
            ui.label(RichText::new(format!("Zone: {}", ztxt)).strong());
            ui.separator();
            let total = state.counters.total_in;
            let col = if total == 0 { egui::Color32::RED } else { egui::Color32::GREEN };
            ui.colored_label(col, RichText::new(format!("Eingebucht: {}", total)).strong());
            if let Some((msg, t, c)) = &state.message {
                if (Local::now() - *t).num_seconds() < 4 {
                    ui.separator();
                    ui.colored_label(*c, msg);
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Beenden").clicked() {
                    app.app_exit();
                }
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.columns(2, |cols| {
            // Left: scan list + input
            let ui_left = &mut cols[0];
            ui_left.heading("Scanner");
            ui_left.horizontal(|uii| {
                let resp = uii.text_edit_singleline(&mut state.input);
                if resp.lost_focus() && uii.input(|i| i.key_pressed(egui::Key::Enter)) {
                    try_add(state, db);
                    uii.memory_mut(|m| m.request_focus(resp.id));
                } else {
                    uii.memory_mut(|m| m.request_focus(resp.id));
                }
            });

            ui_left.separator();
            ui_left.label("Zonen-Knöpfe:");

            // Right: zone grid
            let ui_right = &mut cols[1];
            ui_right.heading("Zonen");
            zone_buttons_grid(ui_right, state, db);
        });
    });
}

fn zone_buttons_grid(ui: &mut egui::Ui, state: &mut AppState, db: &Db) {
    ui.vertical(|ui| {
        ui.horizontal(|uih| {
            zone_btn(uih, state, db, Zone::A);
            zone_btn(uih, state, db, Zone::B);
            zone_btn(uih, state, db, Zone::C);
            zone_btn(uih, state, db, Zone::D);
        });
        ui.add_space(8.0);
        ui.horizontal(|uih| {
            zone_btn(uih, state, db, Zone::E1);
            zone_btn(uih, state, db, Zone::E2);
            zone_btn(uih, state, db, Zone::E3);
            zone_btn(uih, state, db, Zone::E4);
            zone_btn(uih, state, db, Zone::E5);
        });
        ui.add_space(8.0);
        ui.horizontal(|uih| {
            zone_btn(uih, state, db, Zone::E6);
            zone_btn(uih, state, db, Zone::E7);
            zone_btn(uih, state, db, Zone::E8);
            zone_btn(uih, state, db, Zone::E9);
        });
    });
}

fn zone_btn(ui: &mut egui::Ui, state: &mut AppState, db: &Db, zone: Zone) {
    let is_sel = state.current_zone == Some(zone);
    let mut txt = zone.to_string();
    if let Some(n) = state.counters.per_zone.get(&zone) {
        txt.push_str(&format!(" ({})", n));
    }
    let btn = egui::Button::new(RichText::new(txt).strong())
        .rounding(8.0)
        .min_size(egui::vec2(100.0, 48.0));
    let resp = if is_sel {
        ui.add(btn.fill(egui::Color32::from_rgb(30, 120, 200)))
    } else {
        ui.add(btn)
    };
    if resp.clicked() {
        state.current_zone = Some(zone);
        update_counts(state, db);
        state.message = Some((format!("Zone {} gewählt", zone), Local::now(), egui::Color32::LIGHT_BLUE));
    }
}

fn update_counts(state: &mut AppState, db: &Db) {
    let mut sum = 0u64;
    for z in hermes_core::Zone::ALL {
        if let Ok(n) = db.count_in_zone(z) {
            state.counters.per_zone.insert(z, n);
            sum += n;
        }
    }
    state.counters.total_in = sum;
}

fn try_add(state: &mut AppState, db: &Db) {
    let now = Local::now();
    if state.current_zone.is_none() {
        state.message = Some(("Bitte zuerst eine Zone wählen".into(), now, egui::Color32::RED));
        return;
    }
    let tracking = state.input.trim();
    if tracking.is_empty() {
        return;
    }
    // Debounce duplicate scans
    if let Some((last, t)) = &state.last_scan {
        if *last == tracking && (now - *t).num_milliseconds() < state.debounce_ms as i64 {
            state.message = Some(("Doppelscan blockiert".into(), now, egui::Color32::YELLOW));
            state.input.clear();
            return;
        }
    }
    let z = state.current_zone.unwrap();
    match db.add_in(tracking, z) {
        Ok(_) => {
            state.last_scan = Some((tracking.to_string(), now));
            state.input.clear();
            update_counts(state, db);
            state.message = Some((format!("{} → {}", tracking, z), now, egui::Color32::GREEN));
        }
        Err(e) => {
            state.message = Some((format!("Fehler: {}", e), now, egui::Color32::RED));
        }
    }
}
