use super::error::{Error, Result};
use crate::models::user::User;
use sqlx::PgPool;

pub async fn get_user_by_id(db: &PgPool, id: &str) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, role, qr_link, password, created_at, updated_at
        FROM users 
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::RowNotFound) => Err(Error::RowNotFound),
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn get_all_users(db: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, role, qr_link, password, created_at, updated_at 
        FROM users
        "#
    )
    .fetch_all(db)
    .await;

    match users {
        Ok(u) => Ok(u),
        Err(sqlx::Error::RowNotFound) => {
            let u: Vec<User> = Vec::new();
            Ok(u)
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn insert_user(db: &PgPool, u: User) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, name, role, password, qr_link) 
        VALUES ($1, $2, $3, $4, $5) 
        RETURNING id, name, role, qr_link, password, created_at, updated_at
        "#,
        u.id,
        u.name,
        u.role.to_string(),
        u.password,
        u.qr_link
    )
    .fetch_one(db)
    .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::Database(e)) => {
            if e.is_check_violation() {
                Err(Error::UniqueConstraintViolation)
            } else {
                Err(Error::DatabaseError)
            }
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn update_user(db: &PgPool, u: User) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            UPDATE users 
            SET name = $2, role = $3, password = $4, updated_at = NOW() 
            WHERE id = $1 
            RETURNING id, name, role, qr_link, password, created_at, updated_at
            "#,
        u.id,
        u.name,
        u.role.to_string(),
        u.password
    )
    .fetch_one(db)
    .await;

    match user {
        Ok(u) => Ok(u),
        Err(sqlx::Error::Database(e)) => {
            if e.is_check_violation() {
                Err(Error::UniqueConstraintViolation)
            } else {
                Err(Error::DatabaseError)
            }
        }
        Err(_) => Err(Error::DatabaseError),
    }
}

pub async fn delete_user(db: &PgPool, id: &str) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db)
        .await
        .map_err(|_| Error::DatabaseError)?;

    Ok(())
}
