#[rmcp::tool_router]
impl crate::handler::Handler {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool]
    async fn search_crate(&self) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let text = rmcp::model::Content::text("Hello, world!");

        Ok(rmcp::model::CallToolResult::success(vec![text]))
    }
}
