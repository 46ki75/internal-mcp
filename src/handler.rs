#[derive(Debug, Clone)]
pub struct Handler {
    pub tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[rmcp::tool_handler]
impl rmcp::ServerHandler for Handler {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            instructions: None,
            capabilities: rmcp::model::ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: rmcp::model::Implementation {
                name: "cloud.ikuma.internal-mcp".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            ..Default::default()
        }
    }
}
