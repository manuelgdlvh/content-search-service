use lib::infrastructure::app_runner::AppRunner;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AppRunner::run(None).await
}






