use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize, PartialEq)]
#[sqlx(type_name = "role")]
pub enum Role {
    Admin,
    Dosen,
    Mahasiswa,
}
impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => Role::Admin,
            "Dosen" => Role::Dosen,
            "Mahasiswa" => Role::Mahasiswa,
            _ => Role::Mahasiswa,
        }
    }
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub role: Role,
    pub qr_link: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct UserForCreate {
    pub id: String,
    pub name: String,
    pub role: Role,
    pub password: String,
}
impl From<UserForCreate> for User {
    fn from(u: UserForCreate) -> Self {
        User {
            id: u.id,
            name: u.name,
            role: u.role,
            password: u.password,
            qr_link: "".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserForLogin {
    pub id: String,
    pub password: String,
}
