mod app;
mod display;
mod panic;

use crate::panic::initialize_panic_handler;
use anyhow::Result;
use app::BluetoolApp;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler();

    let mut app = BluetoolApp::new();

    app.run().await.unwrap();

    Ok(())
}
