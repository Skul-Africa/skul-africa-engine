use simplelog::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::router::handle_command;

mod router;
mod database;
mod curriculum;
mod utils;

fn init_logger() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("skul_engine.log").unwrap()),
    ]);
}

fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("skul_engine starting...");

    let data_dir = Path::new("data");
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }

// // Remove these lines entirely:
// let db = database::db::init_database("data/engine.db")?;
// database::db::set_global_connection(db);


    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        }

        match handle_command(&line) {
            Ok(resp) => {
                println!("{}", serde_json::to_string(&resp)?);
            }
            Err(e) => {
                let err = serde_json::json!({
                    "status": "error",
                    "error": format!("{}", e)
                });
                println!("{}", err);
            }
        }
    }

    Ok(())
}
