mod data_paths;

pub use data_paths::set_toml_paths_fn as toml_data_paths;
pub use data_paths::TOML_CONFIG_PATHS as TOML_PATHS;

mod env_vars;
pub use env_vars::Configuration as EnvVarsConfig;
