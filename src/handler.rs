#[derive(Debug, Clone)]
pub struct Handler {
    pub tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
    pub resource_map: crate::resource::ResourceMap,
    pub prompt: crate::prompt::Prompt,
}

#[rmcp::tool_handler]
impl rmcp::ServerHandler for Handler {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            instructions: None,
            capabilities: rmcp::model::ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .enable_completions()
                .build(),
            server_info: rmcp::model::Implementation {
                name: "cloud.ikuma.internal-mcp".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }

    fn complete(
        &self,
        _request: rmcp::model::CompleteRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::CompleteResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            // let rmcp::model::ArgumentInfo { name, value } = request.argument;

            let completion = rmcp::model::CompletionInfo {
                has_more: None,
                total: None,
                values: vec!["Ikuma".to_owned()],
            };

            Ok(rmcp::model::CompleteResult { completion })
        }
    }

    fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParam,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::GetPromptResult, rmcp::ErrorData>> + Send + '_
    {
        self.prompt.get_prompt(request, context)
    }

    fn list_prompts(
        &self,
        request: Option<rmcp::model::PaginatedRequestParam>,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListPromptsResult, rmcp::ErrorData>> + Send + '_
    {
        self.prompt.list_prompts(request, context)
    }

    fn list_resources(
        &self,
        request: Option<rmcp::model::PaginatedRequestParam>,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListResourcesResult, rmcp::ErrorData>> + Send + '_
    {
        self.resource_map.list_resources(request, context)
    }

    fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ReadResourceResult, rmcp::ErrorData>> + Send + '_
    {
        self.resource_map.read_resource(request, context)
    }

    fn list_resource_templates(
        &self,
        request: Option<rmcp::model::PaginatedRequestParam>,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListResourceTemplatesResult, rmcp::ErrorData>>
    + Send
    + '_ {
        self.resource_map.list_resource_templates(request, context)
    }
}
