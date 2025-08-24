#[derive(Debug, Clone)]
pub struct Prompt;

impl Prompt {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_prompt(
        &self,
        _request: rmcp::model::GetPromptRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::GetPromptResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            Err(rmcp::ErrorData::method_not_found::<
                rmcp::model::GetPromptRequestMethod,
            >())
        }
    }

    pub fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListPromptsResult, rmcp::ErrorData>> + Send + '_
    {
        async { Ok(rmcp::model::ListPromptsResult::default()) }
    }
}
