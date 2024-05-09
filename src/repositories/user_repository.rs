use sqlx::PgPool;

use crate::models::user::User;
use crate::repositories::error::Result;

pub async fn get_user_by_id(db: &PgPool, id: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, role, password FROM users WHERE id = $1",
        id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name, role, password FROM users")
        .fetch_all(db)
        .await?;

    Ok(users)
}

pub async fn insert_user(db: &PgPool, user: User) -> Result<User> {
    let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, name, role, password) VALUES ($1, $2, $3, $4) RETURNING id, name, role, password",
            user.id,
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(db)
        .await?;

    Ok(user)
}

pub async fn update_user(db: &PgPool, user: User) -> Result<User> {
    let user = sqlx::query_as!(
            User,
            "UPDATE users SET name = $2, role = $3, password = $4 WHERE id = $1 RETURNING id, name, role, password",
            user.id,
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(db)
        .await?;

    Ok(user)
}

pub async fn delete_user(db: &PgPool, id: &str) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db)
        .await?;

    Ok(())
}
