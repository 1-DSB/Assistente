use axum::http::StatusCode;
use axum::{response, Json, Router};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::extract::State;
use candle_transformers::models::quantized_qwen3::ModelWeights;
use tokio;
use serde_json::{json, Value};
use serde::Deserialize;
use tokenizers::Tokenizer;
use crate::model::{Model,llm};


#[derive(Debug)]
enum ApiError {
    NotFound,
    InvallidInput(String),
    InternalError,
}

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub model: Arc<Mutex<ModelWeights>>,
    pub tokenizer: Tokenizer,
}

fn creat_app(state: AppState) -> Router{
    Router::new()
        .route("/health",get(health_checker))
        .route("/api",get(list_users))
        .route("/api/pergunta",post(post_pergunta))
        .with_state(state)
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
async fn post_pergunta(
    State(state): State<AppState>,
    Json(dados): Json<Pergunta>
) -> Result<Json<Value>, ApiError> {

    let mut model = state.model.lock().await;

    let resposta = match llm(
        &mut *model,
        &state.tokenizer,
        dados.pergunta,
        false
    )
    {
        Ok(resposta) => resposta,
        Err(_) => return Err(ApiError::InternalError),
    };
    Ok(Json(json!({
        "resposta": resposta
    })))
}

pub async fn rodar(model: ModelWeights, tokenizer: Tokenizer) {
    let state = AppState {
        model: Arc::new(Mutex::new(model)),
        tokenizer,
    };

    let app = creat_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("falha");

    axum::serve(listener, app)
        .await
        .expect("falha");
}