use crate::{AuthenticatedClient, CreateParserRequest, CreateParserResponse, error::Result};

pub struct AdminApi {
    client: AuthenticatedClient,
}

impl AdminApi {
    pub fn new(client: AuthenticatedClient) -> Self {
        Self { client }
    }

    /// Create a new parser
    pub async fn create_parser(
        &self,
        request: CreateParserRequest,
    ) -> Result<CreateParserResponse> {
        let path = "/admin/parser";
        self.client
            .client
            .post_json(path, Some(&request), Some(&self.client.auth))
            .await
    }

    /// Delete a parser
    pub async fn delete_parser(&self, parser_id: u32) -> Result<()> {
        let path = format!("/admin/parser/{}", parser_id);
        self.client
            .client
            .delete_json(&path, Some(&self.client.auth))
            .await
    }
}
