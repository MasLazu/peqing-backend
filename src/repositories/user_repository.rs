use sqlx::PgPool;

use crate::models::user::User;
use crate::repositories::error::Result;

#[derive(Clone)]
pub struct UserRepository {
    conn: PgPool,
}

impl UserRepository {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, name, role, password FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&self.conn)
        .await?;

        Ok(user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as!(User, r#"SELECT id, name, role, password FROM users"#)
            .fetch_all(&self.conn)
            .await?;

        Ok(users)
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"INSERT INTO users (id, name, role, password) VALUES ($1, $2, $3, $4) RETURNING id, name, role, password"#,
            user.id,
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(&self.conn)
        .await?;

        Ok(user)
    }

    pub async fn update_user(&self, user: User) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"UPDATE users SET name = $2, role = $3, password = $4 WHERE id = $1 RETURNING id, name, role, password"#,
            user.id,
            user.name,
            user.role.to_string(),
            user.password
        )
        .fetch_one(&self.conn)
        .await?;

        Ok(user)
    }
}
