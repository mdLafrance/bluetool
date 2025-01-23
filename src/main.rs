mod app;
mod display;

use anyhow::Result;
use app::BluemanApp;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = BluemanApp::new();

    app.run().await.unwrap();

    Ok(())
}
