use anyhow::Result;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::{query, query_as};

use super::models::User;

use super::pw::HashedPw;

/// `identifier` can be the username OR email
pub async fn get_user(dbc: &mut PoolConnection<Postgres>, identifier: &str) -> Result<User> {
    Ok(query_as!(
        User,
        "SELECT id, username, email FROM users WHERE username = $1 OR email = $1",
        identifier
    )
    .fetch_one(dbc.as_mut())
    .await?)
}

/// Create a user, linked to a row in the `password` table
pub async fn register_user(
    dbc: &mut PoolConnection<Postgres>,
    username: &str,
    email: &str,
    password: &HashedPw,
) -> Result<User> {
    let new_user = query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2)
        RETURNING id, username, email",
        username,
        email
    )
    .fetch_one(dbc.as_mut())
    .await?;

    query!(
        "INSERT INTO password (salt, digest, user_id) VALUES ($1, $2, $3)",
        password.salt,
        password.digest,
        new_user.id
    )
    .execute(dbc.as_mut())
    .await?;

    Ok(new_user)
}
