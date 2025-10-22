
# Hermes RS (Rust Rewrite)

- GUI: egui/eframe (native, fast, touch-friendly)
- DB: SQLite (WAL, busy timeout, crash-safe)
- Logging: tracing
- Config: TOML at OS config dir (e.g., `%AppData%/Roaming/Tannenlaeufer/HermesRS/settings.toml`)

## Build

```
cd hermes_rust
cargo build --release
```

Run GUI:
```
cargo run -p hermes-app
```

Run CLI:
```
cargo run -p hermes-cli -- add A ZXCV1234
```

## Features

- Zonen A–D und E1–E9 mit Debounce gegen Doppelscans
- Eingabefeld behält Fokus, Enter = buchen
- Zähler pro Zone + Gesamt
- SQLite: WAL, busy timeout, idempotente INSERT OR REPLACE
- Keine Platzhalter
