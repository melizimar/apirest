mod util;

use util::gen_random_name;
use util::IdGenerator;

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use time::macros::date;
use time::Date;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum GenderKind {
    Male,
    Famale,
    Nonbinary,
    PreferNotToSay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Person {
    id: usize,
    #[serde(rename = "nome")]
    name: String,
    #[serde(rename = "nascimento")]
    birth_date: Date,
    #[serde(rename = "gênero")]
    gender: GenderKind,
    #[serde(rename = "mãe")]
    mother_name: Option<String>,
    #[serde(rename = "pai")]
    father_name: Option<String>,
}

impl Person {
    fn new(
        id: usize,
        name: String,
        birth_date: Date,
        gender: GenderKind,
        mother_name: Option<String>,
        father_name: Option<String>,
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

    let person1 = Person::new(
        generator.next_id(),
        String::from("Matheus Santos"),
        date!(1999 - 10 - 12),
        GenderKind::Male,
        Some(String::from("Renata Nascimento")),
        Some(String::from("Pedro Santos")),
    );

    let person2 = Person::new(
        generator.next_id(),
        gen_random_name(),
        date!(1986 - 12 - 15),
        GenderKind::Nonbinary,
        Some(gen_random_name()),
        Some(gen_random_name()),
    );

    let person3 = Person::new(
        generator.next_id(),
        gen_random_name(),
        date!(1990 - 09 - 1),
        GenderKind::PreferNotToSay,
        Some(gen_random_name()),
        Some(gen_random_name()),
    );

    let person4 = Person::new(
        generator.next_id(),
        gen_random_name(),
        date!(2005 - 04 - 28),
        GenderKind::Famale,
        Some(gen_random_name()),
        Some(gen_random_name()),
    );

    let person5 = Person::new(
        generator.next_id(),
        gen_random_name(),
        date!(2005 - 04 - 28),
        GenderKind::Famale,
        None,
        Some(gen_random_name()),
    );

    let person6 = Person::new(
        generator.next_id(),
        gen_random_name(),
        date!(2005 - 04 - 28),
        GenderKind::Famale,
        Some(gen_random_name()),
        None,
    );

    permanent_employee.insert(person1.id, person1);
    permanent_employee.insert(person2.id, person2);
    permanent_employee.insert(person3.id, person3);
    permanent_employee.insert(person4.id, person4);
    permanent_employee.insert(person5.id, person5);
    permanent_employee.insert(person6.id, person6);

    let app_state = Arc::new(permanent_employee);

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
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_permanent_employee(
    State(permanent_employee): State<Arc<HashMap<usize, Person>>>,
) -> impl IntoResponse {
    (StatusCode::CREATED, "Buscar -> Servidor Efetivo")
}

async fn read_permanent_employee(
    State(permanent_employee): State<Arc<HashMap<usize, Person>>>,
    Path(employee_id): Path<usize>,
) -> impl IntoResponse {
    match permanent_employee.get(&employee_id) {
        Some(employee) => Ok(Json(employee.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_permanent_employee() -> impl IntoResponse {
    (StatusCode::OK, "Create -> Servidor Efetivo")
}

async fn delete_permanent_employee() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "Delete -> Servidor Efetivo")
}
