use axum::routing::{get, Router};
use std::{
    net::SocketAddr,
    env
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenvy::dotenv;

mod auth;
mod models;
mod db_ops;
mod crypto;
mod pw;
mod session;
mod routes;
mod templates;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Pool<Postgres>
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_conn = create_pg_pool().await;
    let state = AppState {
        db_conn
    };
    let app = Router::new().route("/", get(routes::root))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_pg_pool() -> sqlx::Pool<sqlx::Postgres> {
    let pg_usr =
        &env::var("POSTGRES_USER").expect("postgres user to be defined in environment")[..];
    let pg_pw =
        &env::var("POSTGRES_PASSWORD").expect("postgres password to be defined in environment")[..];
    let pg_db =
        &env::var("POSTGRES_DB").expect("postgres db name to be defined in environment")[..];
    let db_url = &format!("postgres://{}:{}@localhost:5432/{}", pg_usr, pg_pw, pg_db)[..];

    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("pool to startup")
}

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
