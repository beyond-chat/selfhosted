use std::sync::Arc;

use async_openai::types::CreateChatCompletionRequestArgs;
use axum::{extract::State, Json};
use tokio::sync::Mutex;

use crate::http::AppState;
use crate::models::chat::ChatDetails;
use crate::models::llm::{ApiAndModelsConfig, LlmSdk, PromptConfig};
use crate::Result;

// #[axum::debug_handler]
pub async fn create_new_chat(
    State(state): State<AppState>,
    query: String,
) -> Result<Json<ChatDetails>> {
    let pool = &state.pg_pool;

    let title = generate_title(query).await?;

    let details = sqlx::query_as!(
        ChatDetails,
        r#"INSERT INTO ai.chats (title) VALUES ($1) RETURNING *"#,
        title
    )
    .fetch_one(pool)
    .await?;

    // Moka cache invalidate
    let _ = state.cache.chat_history.invalidate(&state.user).await;

    Ok(Json(details))
}

async fn generate_title(query: String) -> Result<String> {
    let llm_cfg = crate::routes::llm_settings::get_selected_fn(0)?;

    Ok(match llm_cfg.0.endpoint_sdk {
        LlmSdk::OpenAI => openai_title(llm_cfg, query).await?,
        LlmSdk::Anthropic => anthropic_title(llm_cfg, query).await?,
    })
}

async fn anthropic_title(
    (api_config, model, prompt_config): (ApiAndModelsConfig, String, PromptConfig),
    query: String,
) -> Result<String> {
    let client = anthropic_sdk::Client::new().auth(api_config.api_key.unwrap().as_str());
    let request = client
            .model(model.as_str())
            .max_tokens(1024i32)
            .temperature(prompt_config.temperature)
            .system("You generate titles with fewer than six words for chatbot messages based on user queries.")
            .messages(&serde_json::json!([{"role": "user", "content": query}]))
            .build()
            .unwrap();
    let title = Arc::new(Mutex::new(String::new()));

    request
        .execute(|text| {
            let my_title = Arc::clone(&title);
            async move {
                let mut title_lock = my_title.lock().await;
                *title_lock = text;
            }
        })
        .await?;

    let final_title = title.lock().await.clone();
    Ok(final_title)
}

async fn openai_title(
    (api_config, model, prompt_config): (ApiAndModelsConfig, String, PromptConfig),
    query: String,
) -> Result<String> {
    let client_config = async_openai::config::OpenAIConfig::new()
        .with_api_key(api_config.api_key.unwrap())
        .with_api_base(api_config.base_url);
    let client = async_openai::Client::with_config(client_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(1024u32)
        .temperature(prompt_config.temperature)
        .messages([
            async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
            .content("You generate titles with fewer than six words for chatbot messages based on user queries.")
            .build()?.into(),
            async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(query)
            .build()?.into(),
        ])
        .build()?;
    Ok(client.chat().create(request).await?.choices[0]
        .message
        .content
        .as_ref()
        .unwrap()
        .to_string())
}
