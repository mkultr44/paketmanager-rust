
mod state;
mod ui;

use anyhow::Result;
use directories::ProjectDirs;
use eframe::egui;
use hermes_db::Db;
use serde::Deserialize;
use state::AppState;
use std::fs;
use std::path::PathBuf;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, Deserialize)]
struct AppCfg {
    app: AppCfgApp,
    storage: AppCfgStorage,
}
#[derive(Debug, Deserialize)]
struct AppCfgApp {
    window_width: u32,
    window_height: u32,
    debounce_ms: u64,
}
#[derive(Debug, Deserialize)]
struct AppCfgStorage {
    db_path: String,
    log_dir: String,
}

fn main() -> Result<()> {
    setup_logging()?;
    let (cfg, data_dir) = load_or_init_cfg()?;
    let db_path = if cfg.storage.db_path.is_empty() {
        data_dir.join("hermes.sqlite3")
    } else { PathBuf::from(&cfg.storage.db_path) };
    let db = Db::open(&db_path)?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([cfg.app.window_width as f32, cfg.app.window_height as f32])
            .with_title("Hermes RS"),
        ..Default::default()
    };

    eframe::run_native(
        "Hermes RS",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_pixels_per_point(1.25);
            let mut state = AppState::new(cfg.app.debounce_ms);
            // Preload counts
            {
                let mut sum = 0u64;
                for z in hermes_core::Zone::ALL {
                    if let Ok(n) = db.count_in_zone(z) {
                        state.counters.per_zone.insert(z, n);
                        sum += n;
                    }
                }
                state.counters.total_in = sum;
            }
            Box::new(MyApp { db, state })
        }),
    )?;
    Ok(())
}

struct MyApp {
    db: Db,
    state: AppState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ui::ui(&mut frame.creation_context(), ctx, &mut self.state, &self.db);
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}

fn setup_logging() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();
    Ok(())
}

fn load_or_init_cfg() -> Result<(AppCfg, PathBuf)> {
    let proj = ProjectDirs::from("de", "Tannenlaeufer", "HermesRS").expect("proj dirs");
    let cfg_dir = proj.config_dir().to_path_buf();
    let data_dir = proj.data_dir().to_path_buf();
    fs::create_dir_all(&cfg_dir)?;
    fs::create_dir_all(&data_dir)?;
    let cfg_path = cfg_dir.join("settings.toml");
    if !cfg_path.exists() {
        let bundled = include_str!("../../../../config/settings.toml");
        fs::write(&cfg_path, bundled)?;
    }
    let s = fs::read_to_string(&cfg_path)?;
    let cfg: AppCfg = toml::from_str(&s)?;
    Ok((cfg, data_dir))
}
