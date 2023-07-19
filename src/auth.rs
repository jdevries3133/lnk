use super::{
    db_ops::get_user,
    pw::{check, HashedPw},
    session::Session,
};
use anyhow::{bail, Result};
use sqlx::{pool::PoolConnection, postgres::Postgres, query_as};

/// `identifier` can be a users username _or_ email
pub async fn authenticate(
    dbc: &mut PoolConnection<Postgres>,
    identifier: &str,
    password: &str,
) -> Result<Session> {
    let user = get_user(dbc, identifier).await?;
    let truth = query_as!(
        HashedPw,
        "SELECT salt, digest FROM password WHERE user_id = $1",
        user.id
    )
    .fetch_one(dbc.as_mut())
    .await?;

    if check(password, &truth).is_ok() {
        Ok(Session { user })
    } else {
        bail!("wrong password")
    }
}
