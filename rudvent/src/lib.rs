mod db;

use axum::{extract::State, routing::get, Router, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use rudvent_lib::get_solutions;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn some_service(State(state): State<MyAxumState>) -> String {
    format!("Hello, {}!", state.name)
}

#[derive(Clone)]
struct MyAxumState {
    pub name: String,
    // add other stuff here
}

async fn adder_endpoint(Path(left): Path<u64>, Path(right): Path<u64>) -> String {
    format!("{} + {} = {}", left, right, rudvent_lib::add(left, right))
}

async fn solutions() -> impl IntoResponse {
    Json(get_solutions())
}

// async fn submit_solution(Json<>)

pub fn build_router(name: String) -> Router {
    let state = MyAxumState {
        name: name,
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/name", get(some_service))
        .route("/add/:left/:right", get(adder_endpoint))
        .route("/solutions", get(solutions))
        .with_state(state)
}