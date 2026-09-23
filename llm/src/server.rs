use axum::http::StatusCode;
use axum::{response, Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use tokio;
use serde_json::{json, Value};
use serde::Deserialize;

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvallidInput(String),
    InternalError,
}

fn creat_app() -> Router{
    Router::new()
        .route("/health",get(health_checker))
        .route("/api",get(list_users))
        .route("/api/pergunta",post(get_pergunta))
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        let (status, error_message) = match self {
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR,
                                        "Erro interno".to_string()),

            ApiError::InvallidInput(msg) => (StatusCode::BAD_REQUEST, msg),

            ApiError::NotFound => (StatusCode::NOT_FOUND,
                                   "Dados não encontrados".to_string())

        };

        let body = Json(json!(
            {"error": error_message,}));

        (status, body).into_response()
    }
}

async fn health_checker() -> impl IntoResponse{
    Json(json!({
        "status":"OK",
        "Message":"OK"
    }))
}

async fn list_users() -> Result<Json<Value>,ApiError>{
    Err(ApiError::InternalError)
}

#[derive(Deserialize)]
struct Pergunta{
    pergunta: String,
}
async fn get_pergunta(Json(dados):Json<Pergunta>) -> Result<Json<Value>,ApiError>{
    let resposta = format!("resposta da pergunta: {}",dados.pergunta);
    println!("{:?}", resposta);
    Ok(Json(json!({
        "resposta":resposta,
        "pergunta":dados.pergunta
    })))

}

pub async fn rodar() {
    let app = creat_app();
    let listener = tokio
    ::net::TcpListener::bind("0.0.0.0:3000").await.expect("falha");
    axum::serve(listener, app).await.expect("falha");
}