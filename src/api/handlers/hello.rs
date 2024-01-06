use crate::state::ApplicationState;
use axum::{extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn hello(State(state): State<Arc<ApplicationState>>) -> Result<String, StatusCode> {
    Ok(format!(
        "\nHello world! Using Configuration from {}\n",
        state
            .settings
            .load()
            .config
            .location
            .clone()
            .unwrap_or("default config".to_string())
    ))
}
