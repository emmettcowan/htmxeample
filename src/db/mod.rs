use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn init_db(url: &str) -> DbPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .expect("failed to connect to db")
}
