use beers::{settings::AppConfig, startup::Application};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let settings = AppConfig::new().expect("Failed to get configruation");
    Application::build(&settings).await.unwrap();
}
