#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
struct GreetParams {
    name: String,
}

#[rmcp::tool_router]
impl crate::handler::Handler {
    pub fn init_tool_router() -> rmcp::handler::server::tool::ToolRouter<Self> {
        Self::tool_router()
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
