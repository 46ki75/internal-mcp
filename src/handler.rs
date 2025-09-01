#[derive(Debug, Clone)]
pub struct Handler {
    pub tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
    pub prompt_router: rmcp::handler::server::router::prompt::PromptRouter<Self>,
    pub resource_map: crate::resource::ResourceMap,
}

use rmcp::{
    RoleServer,
    model::{GetPromptRequestParam, GetPromptResult, ListPromptsResult, PaginatedRequestParam},
    service::RequestContext,
};

impl Handler {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
            resource_map: crate::resource::ResourceMap::new(),
        }
    }
}

#[rmcp::tool_handler]
#[rmcp::prompt_handler]
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
