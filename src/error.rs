use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("API error {status_code}: {message}")]
    Api { status_code: u16, message: String },

    #[error("Serioalization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn from_response(status: u16, body: String) -> Self {
        if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(message) = error_response.get("error").and_then(|v| v.as_str()) {
                return Error::Api {
                    status_code: status,
                    message: message.to_string(),
                };
            }
        }

        Error::Api {
            status_code: status,
            message: body,
        }
    }
}
