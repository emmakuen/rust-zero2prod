use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // initialize logger
    // if RUST_LOG environment variable hasn't been set, fall back to printing all spans at info level or above
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // output the formatted spans to stdout
        std::io::stdout,
    );

    // `with` method is provided by `SubscriberExt`, an extension trait for `Subscriber` exposed by `tracing_subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // the following function can be used to specified what subscriber to be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");

    // panic if config read fails
    let configuration = get_configuration().expect("Failed to read");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let app_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(app_address)?;

    run(listener, connection_pool)?.await
}
