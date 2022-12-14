use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // panic if config read fails
    let configuration = get_configuration().expect("Failed to read configuration");
    let app_address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(app_address)?;
    run(listener)?.await
}
