use crate::{
    AuthenticatedClient, UpdateCallsRequest, UpdateGroupsRequest, UpdateLessonsRequest,
    error::Result,
};

pub struct ParserApi {
    client: AuthenticatedClient,
}

impl ParserApi {
    pub fn new(client: AuthenticatedClient) -> Self {
        Self { client }
    }

    /// Update groups for a campus
    pub async fn update_groups(&self, request: UpdateGroupsRequest) -> Result<()> {
        let path = "/parser/groups";
        self.client
            .client
            .post_json(path, Some(&request), Some(&self.client.auth))
            .await
    }

    /// Update call schedule
    pub async fn update_calls(&self, request: UpdateCallsRequest) -> Result<()> {
        let path = "/parser/calls";
        self.client
            .client
            .post_json(path, Some(&request), Some(&self.client.auth))
            .await
    }

    /// Add lessons
    pub async fn add_lessons(&self, request: UpdateLessonsRequest) -> Result<()> {
        let path = "/parser/lessons";
        self.client
            .client
            .post_json(path, Some(&request), Some(&self.client.auth))
            .await
    }
}
