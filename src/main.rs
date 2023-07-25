use axum::{
    middleware::from_fn,
    routing::{get, post, Router},
};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, net::SocketAddr};

mod auth;
mod components;
mod crypto;
mod db_ops;
mod errors;
mod middleware;
mod models;
mod pw;
mod routes;
mod session;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_conn = create_pg_pool().await;
    let state = AppState { db_conn };
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/login", get(routes::login))
        .route("/login", post(routes::handle_login))
        .route("/logout", post(routes::handle_logout))
        .route("/register", get(routes::register_form))
        .route("/register", post(routes::handle_register))
        .route("/profile", get(routes::get_profile))
        .route("/auth-widget", get(routes::auth_widget_handler))
        .layer(from_fn(middleware::html_headers))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_pg_pool() -> sqlx::Pool<sqlx::Postgres> {
    let pg_usr = &env::var("POSTGRES_USER")
        .expect("postgres user to be defined in environment")[..];
    let pg_pw = &env::var("POSTGRES_PASSWORD")
        .expect("postgres password to be defined in environment")[..];
    let pg_db = &env::var("POSTGRES_DB")
        .expect("postgres db name to be defined in environment")[..];
    let db_url = &format!(
        "postgres://{}:{}@localhost:5432/{}",
        pg_usr, pg_pw, pg_db
    )[..];

    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("pool to startup")
}
