use std::ops::Deref;

use crate::{
    models::llm::{ApiAndModelsConfig, FavModel, PromptConfig},
    utils::init::TOML_PATHS,
    Context, Result,
};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Configs {
    configs: Vec<FavModel>, // configs name need to match the toml array name
}

pub fn get_selected_fn(id: u32) -> Result<(ApiAndModelsConfig, String, PromptConfig)> {
    let toml = read_toml()?;
    let content = toml.configs.iter().find(|config| config.id == id).unwrap();
    let apis = super::api_configs::get_configs_fn()?;
    let selected_api = apis
        .into_iter()
        .find(|api| api.id == content.api_id)
        .expect("Selected API not found");
    let model = content.model.clone();
    let prompt = super::prompt_engineering::get_configs_fn()?;
    let selected_prompt = prompt
        .into_iter()
        .find(|prompt| prompt.id == content.prompt_id)
        .expect("Selected Prompt not found");
    Ok((selected_api, model, selected_prompt))
}

pub async fn get_handler() -> Result<Json<Vec<FavModel>>> {
    let content = read_toml()?;
    Ok(Json(content.configs))
}

pub async fn update_handler(Json(content): Json<FavModel>) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    let index = toml
        .configs
        .iter()
        .position(|config| config.id == content.id);
    if let Some(index) = index {
        toml.configs[index] = content;
    } else {
        toml.configs.push(content);
    }
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

fn read_toml() -> Result<Configs> {
    let content = std::fs::read_to_string(
        TOML_PATHS
            .get()
            .unwrap()
            .llm
            .selected_model_and_prompt
            .deref(),
    )
    .expect("Failed to read prompt-engineering configs from file");
    let toml: Configs = toml::from_str(&content)
        .expect("Failed to parse prompt-engineering configs from toml file");
    Ok(toml)
}
fn save_toml(toml: Configs) -> Result<()> {
    let content =
        toml::to_string(&toml).context("Failed to parse prompt-engineering config to toml")?;
    std::fs::write(
        TOML_PATHS
            .get()
            .unwrap()
            .llm
            .selected_model_and_prompt
            .deref(),
        content,
    )
    .context("Failed to write prompt-engineering config to file")?;
    Ok(())
}
