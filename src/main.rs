mod app;
mod display;

use anyhow::Result;
use app::BTUIApp;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = BTUIApp::new();

    app.run().await.unwrap();

    Ok(())
}
