
use anyhow::Result;
use directories::ProjectDirs;
use hermes_core::Zone;
use hermes_db::Db;

fn main() -> Result<()> {
    let proj = ProjectDirs::from("de","Tannenlaeufer","HermesRS").unwrap();
    let db_path = proj.data_dir().join("hermes.sqlite3");
    let db = Db::open(&db_path)?;
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "add" {
        let zone: Zone = args[2].as_str().try_into().expect("zone");
        let trk = if args.len() >= 4 { &args[3] } else { "TEST123" };
        db.add_in(trk, zone)?;
        println!("OK");
        return Ok(());
    }
    println!("usage: hermes-cli add <ZONE> <TRACKING>");
    Ok(())
}
