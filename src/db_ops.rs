use super::{models::User};
use anyhow::Result;
use sqlx::{pool::PoolConnection, postgres::Postgres, query_as};

/// `identifier` can be the username OR email
pub async fn get_user(
    dbc: &mut PoolConnection<Postgres>,
    identifier: &str,
) -> Result<User> {
    Ok(query_as!(
        User,
        "SELECT id, username, email FROM users WHERE username = $1 OR email = \
         $1",
        identifier
    )
    .fetch_one(dbc.as_mut())
    .await?)
}
