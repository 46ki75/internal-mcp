#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
struct GreetParams {
    name: String,
}

#[rmcp::tool_router]
impl crate::handler::Handler {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            resource_map: crate::resource::ResourceMap::new(),
            prompt: crate::prompt::Prompt::new(),
        }
    }

    #[rmcp::tool]
    async fn greet(
        &self,
        rmcp::handler::server::wrapper::Parameters(GreetParams { name }): rmcp::handler::server::wrapper::Parameters<GreetParams>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let text = rmcp::model::Content::text(format!("Hello, {}", name));

        Ok(rmcp::model::CallToolResult::success(vec![text]))
    }
}
