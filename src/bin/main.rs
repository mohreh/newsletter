use std::net::TcpListener;

use newsletter::{
    configuration::get_configuration,
    run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // refactor database stuff later: todo
    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect_lazy_with(configuration.database.with_db());

    let addr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(addr)?;
    run(listener, pool)?.await
}
