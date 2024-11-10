use std::ops::Deref;

use serde_json::{json, Value};

use crate::{
    models::{cache::CacheState, chat::Chat, llm::PromptConfig},
    Result,
};
pub async fn from_memory(
    (current_query, chat_id, cache, pool, _): (
        String,
        &uuid::Uuid,
        &CacheState,
        &sqlx::PgPool,
        &PromptConfig,
    ),
) -> Result<Value> {
    let chat = Chat::fetch_chat(chat_id, cache, pool).await?;
    let previous_mesages = &chat.read().unwrap().messages;

    let mut messages = vec![];

    for qr in previous_mesages {
        messages.push(json!({"role": "user", "content": qr.user_query.deref()})); // User query
        messages.push(json!({"role": "assistant", "content": qr.assistant_response.deref()}));
        // Assistant response
    }

    // add current query at the end
    messages.push(json!({"role": "user", "content": current_query}));

    Ok(json!(messages))
}
