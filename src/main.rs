mod app;
mod display;
mod panic;

use crate::panic::initialize_panic_handler;
use anyhow::Result;
use app::BTUIApp;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler();

    let mut app = BTUIApp::new();

    app.run().await.unwrap();

    Ok(())
}
