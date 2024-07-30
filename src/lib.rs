mod users;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use users::model::{CreateUserRq, CreateUserRs, GetUserRs};
use users::service::UserService;

pub fn build_router() -> Router {
    let us_builder = UserService::build;

    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/users/:id", get(move |p| get_user(p, us_builder())))
        .route("/users", post(move |p| create_user(p, us_builder())))
}

async fn root() -> &'static str {
    "Hello, Rust!"
}

async fn health() -> (StatusCode, String) {
    (StatusCode::OK, String::from("up"))
}

async fn get_user(Path(id): Path<u64>, service: UserService) -> impl IntoResponse {
    let user = service.get_user(id);
    let res = GetUserRs {
        id: user.id,
        username: user.username,
    };

    (StatusCode::OK, Json(res))
}

async fn create_user(Json(payload): Json<CreateUserRq>, service: UserService) -> impl IntoResponse {
    let user = service.create_user(payload.username);
    let res = CreateUserRs {
        id: user.id,
        username: user.username,
    };

    (StatusCode::CREATED, Json(res))
}
