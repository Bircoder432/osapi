use crate::AdminApi;
use crate::ParserApi;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Auth {
    pub token: Option<String>,
}

impl Auth {
    pub fn new(token: Option<String>) -> Self {
        Self { token }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn clear_token(mut self) -> Self {
        self.token = None;
        self
    }

    pub(crate) fn apply_to_request(
        &self,
        request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            request.bearer_auth(token)
        } else {
            request
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedClient {
    pub client: crate::Client,
    pub auth: Auth,
}

impl AuthenticatedClient {
    pub fn new(client: crate::Client) -> Self {
        Self {
            client,
            auth: Auth::new(None),
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.auth = self.auth.with_token(token);
        self
    }

    pub fn admin(&self) -> AdminApi {
        AdminApi::new(self.clone())
    }

    pub fn parser(&self) -> ParserApi {
        ParserApi::new(self.clone())
    }
}
