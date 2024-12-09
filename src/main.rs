mod util;
use util::gen_random_name;
use util::IdGenerator;

use std::collections::HashMap;

use time::Date;
use time::macros::date;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};

#[warn(dead_code)]
enum GenderKind {
    Male,
    Famale,
    Nonbinary,
    PreferNotToSay,
}

struct Person {
    id: usize,
    name: String,
    birth_date: Date,
    gender: GenderKind,
    mother_name: String,
    father_name: String,
}

impl Person {
    fn new(
        id: usize,
        name: String,
        birth_date: Date,
        gender: GenderKind,
        mother_name: String,
        father_name: String,
    ) -> Self {
        Self {
            id,
            name,
            birth_date,
            gender,
            mother_name,
            father_name,
        }
    }
}

#[tokio::main]
async fn main() {
    let generator = IdGenerator::new(1);
    let mut permanent_employee: HashMap<usize, Person> = HashMap::new();

    let person = Person::new(
        generator.next_id(),
        String::from("Matheus Santos"),
        date!(1999 - 10 - 12),
        GenderKind::Male,
        String::from("Renata Nascimento"),
        String::from("Pedro Santos"),
    );

    permanent_employee.insert(person.id, person);

    let app = Router::new()
        .route("/", get(|| async { "Api Rest" }))
        .route("/efetivo", get(create_permanent_employee))
        .route("/efetivo/:unidade", get(read_permanent_employee))
        .route("/efetivo", post(update_permanent_employee))
        .route("/efetivo", put(delete_permanent_employee))
        // .route("/temporario", get(|| async { gen_random_name() }))
        // .route("/temporario", post(|| async { gen_random_name() }))
        // .route("/temporario", put(|| async { gen_random_name() }))
        // .route("/unidade", get(|| async { gen_random_name() }))
        // .route("/unidade", post(|| async { gen_random_name() }))
        // .route("/unidade", put(|| async { gen_random_name() }))
        // .route("/lotacao", get(|| async { gen_random_name() }))
        // .route("/lotacao", post(|| async { gen_random_name() }))
        // .route("/lotacao", put(|| async { gen_random_name() }))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_permanent_employee() -> impl IntoResponse {
    (StatusCode::CREATED, "Buscar -> Servidor Efetivo")
}

async fn read_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Find -> Servidor Efetivo")
}

async fn update_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Create -> Servidor Efetivo")
}

async fn delete_permanent_employee() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "Delete -> Servidor Efetivo")
}
