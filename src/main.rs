mod util;
use util::gen_random_name;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Api Rest" }))
        .route("/efetivo", get(create_permanent_employee))
        .route("/efetivo/:unidade", get(read_permanent_employee))
        .route("/efetivo", post(update_permanent_employee))
        .route("/efetivo", put(delete_permanent_employee))
        .route("/temporario", get(|| async { gen_random_name() }))
        .route("/temporario", post(|| async { gen_random_name() }))
        .route("/temporario", put(|| async { gen_random_name() }))
        .route("/unidade", get(|| async { gen_random_name() }))
        .route("/unidade", post(|| async { gen_random_name() }))
        .route("/unidade", put(|| async { gen_random_name() }))
        .route("/lotacao", get(|| async { gen_random_name() }))
        .route("/lotacao", post(|| async { gen_random_name() }))
        .route("/lotacao", put(|| async { gen_random_name() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Buscar -> Servidor Efetivo")
}

async fn read_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Find -> Servidor Efetivo")
}

async fn update_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Create -> Servidor Efetivo")
}

async fn delete_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Delete -> Servidor Efetivo")
}
