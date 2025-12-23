//sample query
use crate::db::DbPool;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub async fn get_users(pool: &DbPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
}
