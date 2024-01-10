use loco_rs::cli;
use blockscan_api_wrapper::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
