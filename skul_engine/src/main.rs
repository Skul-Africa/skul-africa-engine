use simplelog::*;
use std::fs::File;
use std::io::{self, BufRead};
use crate::router::handle_command;

mod router;
mod file_parser;
mod student_processor;

fn init_logger() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("skul_engine.log").unwrap()),
    ]);
}

fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("skul_engine starting...");

    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            continue;
        }

        match handle_command(&line) {
            Ok(resp) => println!("{}", serde_json::to_string(&resp)?),
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
