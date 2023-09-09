use std::net::TcpListener;

use newsletter::{
    configuration::get_configuration,
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // refactor database stuff later
    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(addr)?;
    run(listener, pool)?.await
}
