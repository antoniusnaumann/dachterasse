#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_modules;

use shuttle_service::{error::CustomError, ShuttleRocket};
use sqlx::{Executor, PgPool};

pub mod server;

mod cors;
mod database;

#[shuttle_service::main]
async fn init(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleRocket {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    Ok(server::rocket(pool))
}
