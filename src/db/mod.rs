use sqlx::{PgPool, Pool, Postgres};
use crate::config;

pub async fn connect() -> Pool<Postgres> {
    let database_url = config::get_database_url();
    PgPool::connect(&database_url)
        .await
        .expect("Unable to connect to the database")
}
