use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{io::Write, sync::OnceLock};

use crate::models::llm::FavModel;
pub static TOML_CONFIG_PATHS: OnceLock<Paths> = OnceLock::new();

#[derive(Debug)]
pub struct Paths {
    pub config: Config,
    pub llm: Llm,
}

#[derive(Debug)]
pub struct Config {
    pub password_hash: String,
}

#[derive(Debug)]
pub struct Llm {
    pub api_and_models_config: String,
    pub prompt_engineering: String,
    pub selected_model_and_prompt: String,
}
#[derive(Serialize, Deserialize)]
struct FavModelConfigs {
    configs: Vec<FavModel>, // configs name need to match the toml array name
}
pub fn set_toml_paths_fn() -> Result<()> {
    let config = Config {
            password_hash: create_toml("/app/.data/config/admin-user/pass.toml", Some("password_hash = \"$argon2id$v=19$m=19456,t=2,p=1$rcqhKow+Gkq2C1CQMRjdBA$iVoDKImduIYsD3XcaAIMvKeM0JYl9yEaLcx844A7ESg\""))?,
        };
    let default_model = FavModelConfigs {
        configs: vec![FavModel {
            id: 0,
            api_id: 0,
            model: "gpt-3.5-turbo".to_string(),
            prompt_id: 0,
        }],
    };
    let deault_model_content =
        toml::to_string(&default_model).context("Failed to parse default fav model to toml")?;
    let llm = Llm {
        api_and_models_config: create_toml(
            "/app/.data/llm/api_and_models-config.toml",
            Some("configs = []"),
        )?,
        prompt_engineering: create_toml(
            "/app/.data/llm/prompt-engineering.toml",
            Some("configs = []"),
        )?,
        selected_model_and_prompt: create_toml(
            "/app/.data/llm/fav-models.toml",
            Some(&deault_model_content),
        )?,
    };
    TOML_CONFIG_PATHS.set(Paths { config, llm }).unwrap();

    Ok(())
}

fn create_toml(raw_path: &str, default: Option<&str>) -> Result<String> {
    let path = std::path::Path::new(raw_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }
    // Check if the file exists or create a new one with a default value
    if !path.exists() {
        match default {
            Some(default) => {
                let mut file = std::fs::File::create(path).with_context(|| {
                    format!("Failed to create file at path: {}", path.display())
                })?;
                file.write_all(default.as_bytes()).with_context(|| {
                    format!("Failed to write default value to file: {}", path.display())
                })?;
            }
            None => {
                std::fs::File::create(path).with_context(|| {
                    format!("Failed to create file at path: {}", path.display())
                })?;
            }
        }
    }

    // Return path as a string
    Ok(raw_path.to_string())
}
