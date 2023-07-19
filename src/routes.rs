use super::{
    auth::authenticate, errors::ServerError, session::{serialize_session, deserialize_session},
    templates::create_page, utils::html_response, AppState,
    pw,
    models::User
};
use axum::{
    extract::{Form, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use regex::Regex;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, query, query_as, Postgres, Transaction};

/// Generic container for insert queries RETURNING id
struct SqlId {
    id: i32
}

#[derive(Debug)]
struct Name {
    id: i32,
    name: String,
}

pub async fn root(
    State(state): State<AppState>,
) -> Result<Response, ServerError> {
    let mut dbc = get_db(state).await;
    query!(r#"INSERT INTO test (name) VALUES ('tim')"#)
        .execute(dbc.as_mut())
        .await?;
    let _names = query_as!(Name, "SELECT id, name FROM test")
        .fetch_all(dbc.as_mut())
        .await?;
    Ok(html_response(create_page(
        "Home",
        r#"
        <div class='flex flex-col'>
            <button
                hx-get="/login" hx-swap="outerHTML"
                class='bg-blue-100 rounded p-2 m-2 w-24'
            >Login</button>
            <button
                hx-get="/register" hx-swap="outerHTML"
                class='bg-green-100 rounded p-2 m-2 w-24'
            >Register</button>
        </div>
        "#,
    ))
    .into_response())
}

pub async fn login() -> &'static str {
    r#"
        <form hx-post="/login">
            <label>Username: <input type="text" name="username" /></label>
            <label>Password: <input type="password" name="password" /></label>
            <button>Submit</button>
        </form>
   "#
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn handle_login(
    State(state): State<AppState>,
    Form(data): Form<LoginPayload>,
) -> Result<Response, ServerError> {
    let mut dbc = get_db(state).await;
    let session = authenticate(&mut dbc, &data.username, &data.password).await;
    if let Ok(session) = session {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&format!(
                "session={}; Path=/; HttpOnly",
                serialize_session(&session)?
            ))?,
        );
        Ok((headers, "nice").into_response())
    } else {
        println!("Failed login attempt (username = {})", data.username);
        Ok((StatusCode::BAD_REQUEST, "hi").into_response())
    }
}

pub async fn register_form() -> Response {
    r#"
        <form hx-post="/register">
            <label>Email: <input type="text" name="email" /></label>
            <label>Username: <input type="text" name="username" /></label>
            <label>Password: <input type="password" name="password" /></label>
            <button>Submit</button>
        </form>
    "#.into_response()
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn handle_register(
    State(state): State<AppState>,
    Form(data): Form<RegisterPayload>,
) -> Result<Response, ServerError> {
    let mut txn = state.db_conn.begin().await?;
    let hashed_pw = pw::hash_new(&data.password);
    let user = query_as!(
        SqlId,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id",
        data.username,
        data.email
    ).fetch_one(&mut txn).await?;
    query!(
        "INSERT INTO password (user_id, salt, digest) VALUES ($1, $2, $3)",
        user.id,
        hashed_pw.salt,
        hashed_pw.digest
    ).execute(&mut txn).await?;
    txn.commit().await?;

    Ok("you registered".into_response())
}

pub async fn get_profile(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Response, ServerError> {
    let mut dbc = get_db(state).await;
    let user = get_user(headers).await;
    println!("{:?}", user); // wohoo, we have a user
    Ok(r#"
       yo you are
       "#.into_response())
}

async fn get_user(headers: HeaderMap) -> Option<User> {
    let cookie = headers.get("Cookie")?;
    let cookie = cookie.to_str().unwrap_or("");
    let re = Regex::new(r"session=(.*)").unwrap();
    let captures = re.captures(cookie)?;
    let token = &captures[1];
    let deserialize_result = deserialize_session(token);

    if let Ok(session) = deserialize_result {
        Some(session.user)
    } else {
        None
    }
}

async fn get_db(state: AppState) -> PoolConnection<Postgres> {
    state
        .db_conn
        .acquire()
        .await
        .expect("can acquire db connection")
}
