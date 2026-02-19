use crate::api::{CampusesQuery, CollegesQuery, GroupsQuery, ScheduleQuery};
use crate::auth::AuthenticatedClient;
use crate::error::{Error, Result};
use std::time::Duration;

#[cfg(feature = "logging")]
use tracing::{debug, error, instrument};

/// A builder for constructing a Client with custom configuration.
#[derive(Debug)]
pub struct ClientBuilder {
    base_url: String,
    timeout: Duration,
    default_college_id: Option<u32>,
    custom_client: Option<reqwest::Client>,
}

impl ClientBuilder {
    /// Create a new builder with the required base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
            default_college_id: None,
            custom_client: None,
        }
    }

    /// Set the request timeout (default: 30 seconds).
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Set a default college ID for all queries.
    pub fn college_id(mut self, id: u32) -> Self {
        self.default_college_id = Some(id);
        self
    }

    /// Use a custom HTTP client instead of creating a new one.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.custom_client = Some(client);
        self
    }

    /// Build the Client.
    pub fn build(self) -> Client {
        let http_client = self.custom_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(self.timeout)
                .build()
                .expect("Failed to build HTTP client")
        });

        Client {
            base_url: self.base_url.trim_end_matches('/').to_string(),
            http_client,
            default_college_id: self.default_college_id,
        }
    }
}

/// A client for interacting with the OpenScheduleAPI.
///
/// The `Client` provides methods to query colleges, campuses, groups, and schedules.
/// It can be configured with a default college for convenience.
///
/// # Examples
///
/// ```
/// use osars::Client;
///
/// let client = Client::new("https://api.example.com")
///     .with_college(1);
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) base_url: String,
    pub(crate) http_client: reqwest::Client,
    pub(crate) default_college_id: Option<u32>,
}

impl Client {
    /// Create a new builder for constructing a Client.
    pub fn builder(base_url: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(base_url)
    }

    /// Creates a new client with the specified base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API endpoint
    ///
    /// # Examples
    ///
    /// ```
    /// use osars::Client;
    /// let client = Client::new("https://api.example.com");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self::builder(base_url).build()
    }

    /// Returns the base URL of this client.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns the default college ID if set.
    pub fn default_college_id(&self) -> Option<u32> {
        self.default_college_id
    }

    /// Sets a default college ID for subsequent queries.
    ///
    /// # Arguments
    ///
    /// * `college_id` - The ID of the college to set as default
    ///
    /// # Examples
    ///
    /// ```
    /// use osars::Client;
    /// let client = Client::new("https://api.example.com")
    ///     .with_college(1);
    /// ```
    pub fn with_college(mut self, college_id: u32) -> Self {
        self.default_college_id = Some(college_id);
        self
    }

    /// Creates a query to list all colleges.
    ///
    /// # Examples
    ///
    /// ```
    /// use osars::Client;
    /// let client = Client::new("https://api.example.com");
    /// let colleges_query = client.colleges();
    /// ```
    pub fn colleges(&self) -> CollegesQuery<'_> {
        CollegesQuery::new(self)
    }

    /// Creates a query for the default college.
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no default college is set.
    pub fn college(&self) -> Result<crate::api::CollegeQuery<'_>> {
        let college_id = self.default_college_id.ok_or_else(|| {
            Error::Validation("No default college set. Use client.with_college() first".into())
        })?;
        Ok(crate::api::CollegeQuery::new(self, college_id))
    }

    /// Creates a query to list campuses for the default college.
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no default college is set.
    pub fn campuses(&self) -> Result<CampusesQuery<'_>> {
        let college_id = self.default_college_id.ok_or_else(|| {
            Error::Validation("No default college set. Use client.with_college() first".into())
        })?;
        Ok(CampusesQuery::new(self, college_id))
    }

    /// Creates a query for a specific campus.
    ///
    /// Note: This does not require a default college to be set.
    pub fn campus(&self, campus_id: u32) -> crate::api::CampusQuery<'_> {
        crate::api::CampusQuery::new(self, campus_id)
    }

    /// Creates a query to list groups for a campus.
    ///
    /// # Arguments
    ///
    /// * `campus_id` - The ID of the campus
    pub fn groups(&self, campus_id: u32) -> GroupsQuery<'_> {
        GroupsQuery::new(self, campus_id)
    }

    /// Creates a query for a group's schedule.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn schedule(&self, group_id: u32) -> ScheduleQuery<'_> {
        ScheduleQuery::new(self, group_id)
    }

    /// Creates a query for today's schedule of a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn today(&self, group_id: u32) -> ScheduleQuery<'_> {
        self.schedule(group_id).today()
    }

    /// Creates a query for tomorrow's schedule of a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn tomorrow(&self, group_id: u32) -> ScheduleQuery<'_> {
        self.schedule(group_id).tomorrow()
    }

    /// Create an authenticated client for private endpoints.
    pub fn authenticated(self) -> AuthenticatedClient {
        AuthenticatedClient::new(self)
    }

    /// Perform a GET request and deserialize the JSON response.
    #[cfg_attr(feature = "logging", instrument(skip(self), level = "debug"))]
    pub async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        #[cfg(feature = "logging")]
        debug!(%url, "Making GET request");

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(Error::Reqwest)?;

        self.handle_response(response).await
    }

    /// Perform a POST request with JSON body and deserialize the response.
    #[cfg_attr(feature = "logging", instrument(skip(self, body), level = "debug"))]
    pub(crate) async fn post_json<T, B>(
        &self,
        path: &str,
        body: Option<&B>,
        auth: Option<&crate::Auth>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);

        #[cfg(feature = "logging")]
        debug!(%url, "Making POST request");

        let mut request = self.http_client.post(&url);

        if let Some(auth) = auth {
            request = auth.apply_to_request(request);
        }

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(Error::Reqwest)?;
        self.handle_response(response).await
    }

    /// Perform a DELETE request.
    #[cfg_attr(feature = "logging", instrument(skip(self), level = "debug"))]
    pub(crate) async fn delete_json<T>(&self, path: &str, auth: Option<&crate::Auth>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        #[cfg(feature = "logging")]
        debug!(%url, "Making DELETE request");

        let mut request = self.http_client.delete(&url);

        if let Some(auth) = auth {
            request = auth.apply_to_request(request);
        }

        let response = request.send().await.map_err(Error::Reqwest)?;
        self.handle_response(response).await
    }

    /// Handle the HTTP response, checking status and deserializing.
    #[cfg_attr(feature = "logging", instrument(skip(self, response), level = "debug"))]
    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();

        let raw_body = response.text().await.map_err(Error::Reqwest)?;

        #[cfg(feature = "logging")]
        {
            if status.is_success() {
                debug!(status = %status, body = %raw_body, "Request successful");
            } else {
                error!(status = %status, body = %raw_body, "Request failed");
            }
        }

        if status.is_success() {
            // Handle empty responses (e.g., for DELETE)
            if raw_body.is_empty() || raw_body == "null" {
                return serde_json::from_str("null").map_err(|e| {
                    #[cfg(feature = "logging")]
                    error!("Failed to parse empty response: {}", e);
                    Error::Serialization(e)
                });
            }

            serde_json::from_str(&raw_body).map_err(|e| {
                #[cfg(feature = "logging")]
                error!(error = %e, body = %raw_body, "JSON parse error");
                Error::Serialization(e)
            })
        } else {
            Err(Error::from_response(status.as_u16(), raw_body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, ServerOpts};

    #[test]
    fn test_client_creation() {
        let client = Client::new("https://api.example.com");
        assert_eq!(client.base_url, "https://api.example.com");
        assert!(client.default_college_id.is_none());
    }

    #[test]
    fn test_client_builder() {
        let client = Client::builder("https://api.example.com")
            .college_id(123)
            .timeout(Duration::from_secs(60))
            .build();

        assert_eq!(client.base_url, "https://api.example.com");
        assert_eq!(client.default_college_id, Some(123));
    }

    #[test]
    fn test_client_with_college() {
        let client = Client::new("https://api.example.com").with_college(123);
        assert_eq!(client.default_college_id, Some(123));
    }

    #[test]
    fn test_client_url_trimming() {
        let client = Client::new("https://api.example.com/");
        assert_eq!(client.base_url, "https://api.example.com");

        let client2 = Client::new("https://api.example.com//");
        assert_eq!(client2.base_url, "https://api.example.com");
    }

    #[test]
    fn test_college_without_default() {
        let client = Client::new("https://api.example.com");
        let result = client.college();
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation(msg) => assert!(msg.contains("default college")),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_college_with_default() {
        let client = Client::new("https://api.example.com").with_college(123);
        let result = client.college();
        assert!(result.is_ok());
    }

    #[test]
    fn test_campuses_without_default() {
        let client = Client::new("https://api.example.com");
        let result = client.campuses();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_json_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "test"}"#)
            .create_async()
            .await;

        let client = Client::new(&server.url());
        let result: serde_json::Value = client.get_json("/test").await.unwrap();

        mock.assert_async().await;
        assert_eq!(result["name"], "test");
    }

    #[tokio::test]
    async fn test_get_json_api_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(404)
            .with_body(r#"{"error": "Not found"}"#)
            .create_async()
            .await;

        let client = Client::new(&server.url());
        let result: Result<serde_json::Value> = client.get_json("/test").await;

        mock.assert_async().await;
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::NotFound(msg) => assert_eq!(msg, "Not found"),
            other => panic!("Expected NotFound, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_post_json_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/test")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1}"#)
            .create_async()
            .await;

        let client = Client::new(&server.url());
        let body = serde_json::json!({"name": "test"});
        let result: serde_json::Value = client.post_json("/test", Some(&body), None).await.unwrap();

        mock.assert_async().await;
        assert_eq!(result["id"], 1);
    }

    #[tokio::test]
    async fn test_delete_json_empty_response() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("DELETE", "/test/1")
            .with_status(204)
            .with_body("")
            .create_async()
            .await;

        let client = Client::new(&server.url());
        let result: serde_json::Value = client.delete_json("/test/1", None).await.unwrap();

        mock.assert_async().await;
        assert!(result.is_null());
    }
}
