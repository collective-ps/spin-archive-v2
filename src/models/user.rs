use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

use crate::config;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub role: i16,
    pub daily_upload_limit: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_by_username(db: &PgPool, username: &str) -> anyhow::Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
	SELECT id, username, password_hash, email, role, daily_upload_limit, created_at, updated_at
	FROM users
	WHERE username = $1
	"#,
        username
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn login(
    db: &PgPool,
    login_username: &str,
    login_password: &str,
) -> Result<User, LoginError> {
    let user = get_by_username(&db, &login_username.to_lowercase())
        .await
        .map_err(|_| LoginError::InvalidPasswordOrUser)?;

    if verify_password(login_password, &user.password_hash) {
        Ok(user)
    } else {
        Err(LoginError::InvalidPasswordOrUser)
    }
}

pub enum LoginError {
    InvalidPasswordOrUser,
}

#[allow(dead_code)]
pub enum RegistrationError {
    PasswordFailure,
    AlreadyExists,
}

#[allow(dead_code)]
fn hash_password(password: &str) -> Result<String, RegistrationError> {
    let salt = config::secret_key();
    let config = argon2::Config::default();

    argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config)
        .map_err(|_| RegistrationError::PasswordFailure)
}

fn verify_password(password: &str, hash: &str) -> bool {
    match argon2::verify_encoded(hash, password.as_ref()) {
        Ok(_) => true,
        _ => false,
    }
}
