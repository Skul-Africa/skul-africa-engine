use simplelog::*;
use std::fs::File;
use tokio::net::TcpListener;
use dotenvy::dotenv;

mod router;
mod file_parser;
mod student_processor;
mod server;
mod school_form_parser;
mod auth;

fn init_logger() {
    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_location_level(LevelFilter::Off)
        .build();

    let log_file = File::create("skul_engine.log").unwrap_or_else(|err| {
        eprintln!("Failed to create log file: {}", err);
        std::process::exit(1);
    });

    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, config, log_file),
    ])
    .unwrap_or_else(|err| {
        eprintln!("Failed to initialize logger: {}", err);
        std::process::exit(1);
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // now it will work

    init_logger();
    log::info!("skul_engine API starting...");

    let app = server::create_router();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
