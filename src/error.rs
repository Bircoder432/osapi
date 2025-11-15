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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_response_with_json() {
        let body = r#"{"error": "Not found"}"#.to_string();
        let error = Error::from_response(404, body);

        match error {
            Error::Api {
                status_code,
                message,
            } => {
                assert_eq!(status_code, 404);
                assert_eq!(message, "Not found");
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[test]
    fn test_error_from_response_with_plain_text() {
        let body = "Internal Server Error".to_string();
        let error = Error::from_response(500, body);

        match error {
            Error::Api {
                status_code,
                message,
            } => {
                assert_eq!(status_code, 500);
                assert_eq!(message, "Internal Server Error");
            }
            _ => panic!("Expected Api error"),
        }
    }
}
