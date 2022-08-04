#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_modules;

use shuttle_service::ShuttleRocket;
use sqlx::PgPool;

pub mod server;

mod database;

#[shuttle_service::main]
async fn init(#[shared::Postgres] pool: PgPool) -> ShuttleRocket {
    Ok(server::rocket(pool))
}
