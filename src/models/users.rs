use std::borrow::Cow;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Decode, FromRow, PgPool, Row};
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUser<'a> {
    username: Cow<'a, str>,
    email: Cow<'a, str>,
    password: Cow<'a, str>,
    confirm_password: Cow<'a, str>,
}

impl<'a> RegisterUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password: &'a str, confirm: &'a str) -> Self {
        Self {
            username: Cow::Borrowed(username),
            email: Cow::Borrowed(email),
            password: Cow::Borrowed(password),
            confirm_password: Cow::Borrowed(confirm),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilteredUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl From<User> for FilteredUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at.format("%d-%m-%Y %H:%M").to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Decode, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<FixedOffset>,
}

impl User {
    #[tracing::instrument(skip_all)]
    pub async fn register(db: &PgPool, dto: &RegisterUser<'_>) -> Result<Self> {
        let password = Self::hash_password(&dto.password)?;
        // Create a Transaction to perform multiple queries on one connection
        let mut txn = db.begin().await?;

        // E-mail and username must be unique
        let exists: Option<PgRow> =
            sqlx::query("SELECT email, username FROM users WHERE email = $1 OR username = $2")
                .bind(&dto.email)
                .bind(&dto.username)
                .fetch_optional(&mut *txn)
                .await?;

        if let Some(row) = exists {
            let email_exists: &str = row.try_get("email").unwrap_or_default();

            if email_exists == &dto.email {
                return Err(
                    Error::EntityAlreadyExists("User with email already exists".into()).into(),
                );
            } else {
                return Err(Error::EntityAlreadyExists("Username already taken".into()).into());
            }
        }

        let user = sqlx::query_as::<_, Self>(
            "INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&dto.email)
        .bind(&dto.username)
        .bind(password)
        .fetch_one(&mut *txn)
        .await?;

        Ok(user)
    }

    pub fn hash_password(password: &str) -> Result<String> {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hashed = argon.hash_password(password.as_bytes(), &salt);

        if let Ok(hashed) = hashed {
            Ok(hashed.to_string())
        } else {
            let err = Error::from(hashed.unwrap_err());
            Err(err.into())
        }
    }
}
