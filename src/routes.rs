use super::templates::create_page;
use axum::{
    response::IntoResponse,
    extract::State
};
use sqlx::{query, query_as, pool::PoolConnection, Postgres};

use super::AppState;
use super::utils::html_response;

struct Name {
    id: i32,
    name: String
}

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    let mut dbc = get_db(state).await;
    query!(r#"INSERT INTO test (name) VALUES ('tim')"#).execute(dbc.as_mut()).await.expect("can insert");
    let name = query_as!(
        Name,
        "SELECT id, name FROM test"
    ).fetch_one(dbc.as_mut()).await.expect("can fetch name");
    html_response(create_page("Home", &format!("heyo, {}::{}", name.id, name.name)))
}

async fn get_db(state: AppState) -> PoolConnection<Postgres> {
    state.db_conn.acquire().await.expect("can get a database connection")
}
