mod anthropic;
mod anthropic_convo;
mod openai;
mod openai_convo;

use axum::{
    extract::{Query, State},
    response::Response,
    routing::get,
    Router,
};

use crate::Result;
use crate::{http::AppState, models::llm::LlmSdk};

pub fn router() -> Router<AppState> {
    Router::new().route("/chat-sse", get(handler))
}

#[derive(serde::Deserialize)]
struct LlmType {
    chat_id: uuid::Uuid,
    access_token: String,
    selected_fav_model_id: u32,
    query: String,
}

async fn handler(State(state): State<AppState>, Query(params): Query<LlmType>) -> Result<Response> {
    let _verify_jwt = crate::http::middleware::JWT::decode(params.access_token)?;
    let llm_cfg = super::llm_settings::get_selected_fn(params.selected_fav_model_id)?;

    let message_params = (
        params.query,
        &params.chat_id,
        &state.cache,
        &state.pg_pool,
        &llm_cfg.2,
    );
    Ok(match llm_cfg.0.endpoint_sdk {
        LlmSdk::OpenAI => {
            let messages = openai_convo::from_memory(message_params).await?;
            openai::client_request(llm_cfg, messages).await?
        }
        LlmSdk::Anthropic => {
            let messages = anthropic_convo::from_memory(message_params).await?;
            anthropic::client_request(llm_cfg, &messages).await?
        }
    })
}
// Ok(match llm_type.as_str() {
//     "openai" => openai::stream(llm_api.get("openai").unwrap(), messages).await,
//     "hyperbolic" => hyperbolic::stream(llm_api.get("hyperbolic").unwrap(), messages).await,
//     _ => (StatusCode::BAD_REQUEST, "Unknown LLM type").into_response(),
// })
