use loco_rs::cli;
use trolley::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
