use simplelog::*;
use std::fs::File;
use tokio::net::TcpListener;

mod router;
mod file_parser;
mod student_processor;
mod server;

fn init_logger() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("skul_engine.log").unwrap()),
    ]);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();
    log::info!("skul_engine API starting...");

    let app = server::create_router();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
