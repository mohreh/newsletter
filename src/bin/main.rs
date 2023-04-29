use std::net::TcpListener;

use newsletter::{
    configuration::get_configuration,
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into());
    init_subscriber(subscriber);

    // refactor database stuff later
    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addr)?;
    run(listener, pool)?.await
}
