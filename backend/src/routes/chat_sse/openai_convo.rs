use std::ops::Deref;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};

use crate::{
    models::{cache::CacheState, chat::Chat, llm::PromptConfig},
    Result,
};

pub async fn from_memory(
    (current_query, chat_id, cache, pool, prompt_cfg): (
        String,
        &uuid::Uuid,
        &CacheState,
        &sqlx::PgPool,
        &PromptConfig,
    ),
) -> Result<Vec<ChatCompletionRequestMessage>> {
    let chat = Chat::fetch_chat(chat_id, cache, pool).await?;
    let previous_mesages = &chat.read().unwrap().messages;

    let mut messages: Vec<ChatCompletionRequestMessage> =
        vec![ChatCompletionRequestSystemMessageArgs::default()
            .content(prompt_cfg.system_prompt.deref())
            .build()?
            .into()];

    for qr in previous_mesages {
        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(qr.user_query.deref())
                .build()?
                .into(),
        );
        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(qr.assistant_response.deref())
                .build()?
                .into(),
        );
    }

    // add current query at the end
    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(current_query)
            .build()?
            .into(),
    );

    Ok(messages)
}
