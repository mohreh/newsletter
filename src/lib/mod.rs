use std::net::TcpListener;

use actix_web::dev::Server;
use sqlx::PgPool;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    startup::run(listener, pool)
}
