use crate::Auth;
use crate::api::{CampusQuery, CampusesQuery, CollegeQuery, CollegesQuery};
use crate::auth::AuthenticatedClient;
use crate::error::Result;
use crate::{GroupsQuery, ScheduleQuery, error::Error};
/// A client for interacting with the educational schedule API.
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
#[cfg(feature = "logging")]
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) base_url: String,
    pub http_client: reqwest::Client,
    pub(crate) default_college_id: Option<u32>,
}

impl Client {
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
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
            default_college_id: None,
        }
    }

    /// Creates a new client with a custom HTTP client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API endpoint
    /// * `http_client` - Custom reqwest client for advanced configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Client;
    /// use osars::Client as osarsClient;
    ///
    /// let http_client = Client::builder()
    ///     .timeout(std::time::Duration::from_secs(30))
    ///     .build()
    ///     .unwrap();
    ///
    /// let client = osarsClient::with_client("https://api.example.com", http_client);
    /// ```
    pub fn with_client(base_url: &str, http_client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client,
            default_college_id: None,
        }
    }

    // Sets a default college ID for subsequent queries.
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
    pub fn colleges(&self) -> CollegesQuery {
        CollegesQuery::new(self)
    }

    /// Creates a query for the default college.
    ///
    /// # Returns
    ///
    /// Returns `Ok(CollegeQuery)` if a default college is set, otherwise `Err(Error)`.
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no default college is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use osars::Client;
    /// let client = Client::new("https://api.example.com")
    ///     .with_college(1);
    /// let college_query = client.college().unwrap();
    /// ```
    pub fn college(&self) -> Result<CollegeQuery> {
        let college_id = self.default_college_id.ok_or_else(|| {
            Error::Validation("No default college set. Use client.with_college() first".into())
        })?;
        Ok(CollegeQuery::new(self, college_id))
    }

    /// Creates a query to list campuses for the default college.
    ///
    /// # Returns
    ///
    /// Returns `Ok(CampusesQuery)` if a default college is set, otherwise `Err(Error)`.
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no default college is set.
    pub fn campuses(&self) -> Result<CampusesQuery> {
        let college_id = self
            .default_college_id
            .ok_or_else(|| Error::Validation("No default college set".into()))?;
        Ok(CampusesQuery::new(self, college_id))
    }

    /// Creates a query for a specific campus.
    ///
    /// # Arguments
    ///
    /// * `campus_id` - The ID of the campus to query
    ///
    /// # Returns
    ///
    /// Returns `Ok(CampusQuery)` if a default college is set, otherwise `Err(Error)`.
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if no default college is set.
    pub fn campus(&self, campus_id: u32) -> Result<CampusQuery> {
        let _ = self.default_college_id.ok_or_else(|| {
            Error::Validation("No default college set. Use client.with_college() first".into())
        })?;
        Ok(CampusQuery::new(self, campus_id))
    }

    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(feature = "logging")]
        debug!("GET {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(crate::error::Error::Reqwest)?;

        let status = response.status();

        #[cfg(feature = "logging")]
        {
            let headers = response.headers();
            debug!("Response headers: {:#?}", headers);
        }

        let raw_body = response
            .text()
            .await
            .map_err(crate::error::Error::Reqwest)?;

        #[cfg(feature = "logging")]
        {
            if status.is_success() {
                debug!("Success {}: raw response = {}", status, raw_body);
            } else {
                error!("API error {}: raw response = {}", status, raw_body);
            }
        }

        if status.is_success() {
            serde_json::from_str(&raw_body).map_err(|e| {
                #[cfg(feature = "logging")]
                error!("JSON parse error: {}\nRaw body: {}", e, raw_body);
                crate::error::Error::Serialization(e)
            })
        } else {
            Err(crate::error::Error::from_response(
                status.as_u16(),
                raw_body,
            ))
        }
    }
    pub(crate) async fn post_json<T, B>(
        &self,
        path: &str,
        body: Option<&B>,
        auth: Option<&Auth>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(feature = "logging")]
        debug!("POST {}", url);

        let mut request = self.http_client.post(&url);

        if let Some(auth) = auth {
            request = auth.apply_to_request(request);
        }

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await.map_err(crate::error::Error::Reqwest)?;

        self.handle_response(response).await
    }

    pub(crate) async fn delete_json<T>(&self, path: &str, auth: Option<&Auth>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        #[cfg(feature = "logging")]
        debug!("DELETE {}", url);

        let mut request = self.http_client.delete(&url);

        if let Some(auth) = auth {
            request = auth.apply_to_request(request);
        }

        let response = request.send().await.map_err(crate::error::Error::Reqwest)?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let raw_body = response
            .text()
            .await
            .map_err(crate::error::Error::Reqwest)?;

        #[cfg(feature = "logging")]
        {
            if status.is_success() {
                debug!("Success {}: raw response = {}", status, raw_body);
            } else {
                error!("API error {}: raw response = {}", status, raw_body);
            }
        }

        if status.is_success() {
            if raw_body.is_empty() {
                // Handle empty response for DELETE and some POST requests
                serde_json::from_str("null").map_err(|e| {
                    #[cfg(feature = "logging")]
                    error!("JSON parse error for empty response: {}", e);
                    crate::error::Error::Serialization(e)
                })
            } else {
                serde_json::from_str(&raw_body).map_err(|e| {
                    #[cfg(feature = "logging")]
                    error!("JSON parse error: {}\nRaw body: {}", e, raw_body);
                    crate::error::Error::Serialization(e)
                })
            }
        } else {
            Err(crate::error::Error::from_response(
                status.as_u16(),
                raw_body,
            ))
        }
    }

    /// Creates a query to list groups for a campus.
    ///
    /// # Arguments
    ///
    /// * `campus_id` - The ID of the campus
    pub fn groups(&self, campus_id: u32) -> GroupsQuery {
        GroupsQuery::new(self, campus_id)
    }

    /// Creates a query for a group's schedule.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn schedule(&self, group_id: u32) -> ScheduleQuery {
        ScheduleQuery::new(self, group_id)
    }

    /// Creates a query for today's schedule of a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn today(&self, group_id: u32) -> ScheduleQuery {
        self.schedule(group_id).today()
    }

    /// Creates a query for tomorrow's schedule of a group.
    ///
    /// # Arguments
    ///
    /// * `group_id` - The ID of the student group
    pub fn tomorrow(&self, group_id: u32) -> ScheduleQuery {
        self.schedule(group_id).tomorrow()
    }
    /// Create an authenticated client for private endpoints
    pub fn authenticated(&self) -> AuthenticatedClient {
        AuthenticatedClient::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{self, Server};

    #[test]
    fn test_client_creation() {
        let client = Client::new("https://api.example.com");
        assert_eq!(client.base_url, "https://api.example.com");
        assert!(client.default_college_id.is_none());
    }

    #[test]
    fn test_client_with_college() {
        let client = Client::new("https://api.example.com").with_college(123);
        assert_eq!(client.default_college_id, Some(123));
    }

    #[test]
    fn test_client_with_custom_http_client() {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        let client = Client::with_client("https://api.example.com", http_client);
        assert_eq!(client.base_url, "https://api.example.com");
    }

    #[test]
    fn test_college_without_default() {
        let client = Client::new("https://api.example.com");
        let result = client.college();
        assert!(result.is_err());
    }

    #[test]
    fn test_college_with_default() {
        let client = Client::new("https://api.example.com").with_college(123);
        let result = client.college();
        assert!(result.is_ok());
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
    }
}
