use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("API error {status_code}: {message}")]
    Api { status_code: u16, message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn from_response(status: u16, body: String) -> Self {
        // Try to parse JSON error
        let message = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            json.get("error")
                .and_then(|e| e.as_str())
                .or_else(|| json.get("message").and_then(|m| m.as_str()))
                .map(String::from)
                .unwrap_or(body)
        } else {
            body
        };

        match status {
            401 | 403 => Error::Unauthorized,
            404 => Error::NotFound(message),
            422 => Error::Validation(message),
            _ => Error::Api {
                status_code: status,
                message,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_response_with_json_error_field() {
        let body = r#"{"error": "College not found"}"#.to_string();
        let error = Error::from_response(404, body);

        match error {
            Error::NotFound(msg) => assert_eq!(msg, "College not found"),
            _ => panic!("Expected NotFound error, got {:?}", error),
        }
    }

    #[test]
    fn test_error_from_response_with_json_message_field() {
        let body = r#"{"message": "Invalid parameters"}"#.to_string();
        let error = Error::from_response(422, body);

        match error {
            Error::Validation(msg) => assert_eq!(msg, "Invalid parameters"),
            _ => panic!("Expected Validation error, got {:?}", error),
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

    #[test]
    fn test_error_from_response_unauthorized() {
        let body = r#"{"error": "Invalid token"}"#.to_string();
        let error = Error::from_response(401, body);

        match error {
            Error::Unauthorized => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_error_display_formats() {
        let err = Error::Validation("test message".to_string());
        assert_eq!(err.to_string(), "Validation error: test message");

        let err = Error::NotFound("resource".to_string());
        assert_eq!(err.to_string(), "Not found: resource");

        let err = Error::Unauthorized;
        assert_eq!(err.to_string(), "Authentication required");
    }
}
