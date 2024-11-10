use std::convert::Infallible;

use anthropic_sdk::Client;
use axum::response::sse::{Event, Sse};
use axum::response::{IntoResponse, Response};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};

use crate::{
    models::llm::{ApiAndModelsConfig, PromptConfig},
    Result,
};

pub async fn client_request(
    (api_config, model, prompt_config): (ApiAndModelsConfig, String, PromptConfig),
    messages: &serde_json::Value,
) -> Result<Response> {
    let client = Client::new().auth(api_config.api_key.unwrap().as_str());
    let request = client
        .model(model.as_str())
        .max_tokens(prompt_config.max_tokens as i32)
        .temperature(prompt_config.temperature)
        .system(&prompt_config.system_prompt)
        .messages(messages)
        .stream(true)
        .build()
        .unwrap();

    Ok(create_sse(request).await.into_response())
}

async fn create_sse(
    request: anthropic_sdk::Request,
) -> Sse<impl Stream<Item = core::result::Result<Event, Infallible>>> {
    //Uses mpsc::channel instead of direct stream mapping since Anthropic's SDK uses callbacks
    // Channel to communicate the messages from the stream
    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(100);

    tokio::spawn(async move {
        let tx_clone = tx.clone();
        // Execute the Anthropic stream request
        if let Err(error) = request
            .execute(move |text| {
                let tx = tx_clone.clone();
                async move {
                    if let Err(e) = tx.send(Ok(Event::default().data(text))).await {
                        tracing::error!("Failed to send event: {}", e);
                    }
                }
            })
            .await
        {
            tracing::error!("Error executing Anthropic stream: {}", error);
            // Send an error event
            if let Err(e) = tx.send(Ok(Event::default().data("End of Stream"))).await {
                tracing::error!("Failed to send error event: {}", e);
            }
        }
    });

    // Convert receiver into a stream
    let event_stream = ReceiverStream::new(rx);

    // Convert the stream into sse response
    Sse::new(event_stream)
}
