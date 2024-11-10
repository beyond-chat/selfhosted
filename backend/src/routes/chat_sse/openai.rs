use std::convert::Infallible;

use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
        FinishReason,
    },
    Client,
};
use axum::response::{sse::Event, IntoResponse, Response, Sse};
use tokio_stream::{Stream, StreamExt};

use crate::{
    models::llm::{ApiAndModelsConfig, PromptConfig},
    Result,
};

pub async fn client_request(
    (api_config, model, prompt_config): (ApiAndModelsConfig, String, PromptConfig),
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<Response> {
    let client_config = OpenAIConfig::new()
        .with_api_key(api_config.api_key.unwrap())
        .with_api_base(api_config.base_url);
    let client = Client::with_config(client_config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .max_tokens(prompt_config.max_tokens)
        .temperature(prompt_config.temperature)
        .messages(messages)
        .stream(true)
        .build()?;

    Ok(create_sse(&client, request).await.into_response())
}

async fn create_sse(
    client: &Client<OpenAIConfig>,
    request: CreateChatCompletionRequest,
) -> Sse<impl Stream<Item = core::result::Result<Event, Infallible>>> {
    let openai_crate_stream = client.chat().create_stream(request).await.unwrap();

    // Transform OpenAI stream into SSE events
    let event_stream = openai_crate_stream.map(|result| {
        match result {
            Ok(response) => match response.choices.first() {
                Some(chat_choice) => match chat_choice.finish_reason {
                    Some(finish_reason) => {
                        if finish_reason != FinishReason::Stop {
                            tracing::debug!("Finish reason: {:?}", finish_reason);
                        }
                        Ok(Event::default().data("End of Stream"))
                    } // End stream with finish reason
                    None => {
                        // let else_content = String::from("This implies content is None but that wont happen since we check for it above in find");
                        let else_content = String::from("End of Stream");
                        let content = chat_choice.delta.content.as_ref().unwrap_or(&else_content);
                        Ok(Event::default().data(content))
                    }
                },
                None => {
                    tracing::debug!("No choices found");
                    Ok(Event::default().data("End of Stream"))
                }
            },
            Err(err) => {
                tracing::error!("SSE Error: {:?}", err);
                Ok(Event::default().data("End of Stream"))
            }
        }
    });

    Sse::new(event_stream)
}
