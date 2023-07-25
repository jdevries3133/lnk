use super::{
    auth::authenticate,
    components::{
        auth_widget, create_page, form_field, hx_form, unauthenticated_actions,
        AuthWidgetProps, CreatePageProps, FormFieldProps, HxFormProps,
    },
    errors::ServerError,
    models::{SqlId, User},
    pw,
    session::{deserialize_session, serialize_session},
    utils::html_response,
    AppState,
};
use axum::{
    extract::{Form, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use regex::Regex;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, query, query_as, Postgres};

pub async fn root(headers: HeaderMap) -> impl IntoResponse {
    let user = get_user(headers).await;
    html_response(create_page(CreatePageProps::new(
        "Home",
        &auth_widget(AuthWidgetProps { user }),
    )))
}

pub async fn auth_widget_handler(headers: HeaderMap) {
    let user = get_user(headers).await;
    auth_widget(AuthWidgetProps { user });
}

pub async fn login() -> impl IntoResponse {
    format!(
        r#"<div class="w-48">{}</div>"#,
        hx_form(HxFormProps {
            children: [
                form_field(FormFieldProps {
                    input_type: "text".into(),
                    name: "username".into(),
                    label_text: "Username".into(),
                }),
                form_field(FormFieldProps {
                    input_type: "password".into(),
                    name: "password".into(),
                    label_text: "Password".into(),
                }),
            ]
            .join(""),
            hx_post: "/login",
            hx_target: "#auth-widget"
        })
    )
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
        Ok((headers, auth_widget(AuthWidgetProps { user: Some(session.user) })).into_response())
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
    "#
    .into_response()
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
    )
    .fetch_one(&mut txn)
    .await?;
    query!(
        "INSERT INTO password (user_id, salt, digest) VALUES ($1, $2, $3)",
        user.id,
        hashed_pw.salt,
        hashed_pw.digest
    )
    .execute(&mut txn)
    .await?;
    txn.commit().await?;

    Ok("you registered".into_response())
}

pub async fn handle_logout() -> Result<impl IntoResponse, ServerError> {
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str("null")?);
    Ok((
        headers,
        format!("<p>You have logged out!</p>{}", unauthenticated_actions()),
    ))
}

pub async fn get_profile(headers: HeaderMap) -> Result<Response, ServerError> {
    let user = get_user(headers).await;
    println!("{:?}", user);
    Ok(r#"
       yo you are
       "#
    .into_response())
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
