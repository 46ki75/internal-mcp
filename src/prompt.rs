#[derive(Debug, Clone)]
pub struct Prompt {
    pub prompts: std::collections::HashMap<String, std::sync::Arc<dyn IntoMcpPrompt>>,
}

#[derive(Debug, Clone)]
pub struct TemplatedPrompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<rmcp::model::PromptArgument>>,
    pub template: String,
}

pub trait IntoMcpPrompt: std::fmt::Debug + Send + Sync {
    fn name(&self) -> String;
    fn into_get_result(
        &self,
        arguments: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> rmcp::model::GetPromptResult;
    fn into_list_result(&self) -> rmcp::model::Prompt;
}

impl IntoMcpPrompt for TemplatedPrompt {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn into_get_result(
        &self,
        arguments: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> rmcp::model::GetPromptResult {
        let message = if let Some(arguments) = arguments {
            let mut message: String = self.template.clone();
            for (k, v) in arguments {
                message = message.replace(&format!("{{{}}}", k), &v.to_string());
            }
            message
        } else {
            self.template.clone()
        };

        rmcp::model::GetPromptResult {
            description: self.description.to_owned(),
            messages: vec![rmcp::model::PromptMessage {
                role: rmcp::model::PromptMessageRole::User,
                content: rmcp::model::PromptMessageContent::text(message),
            }],
        }
    }

    fn into_list_result(&self) -> rmcp::model::Prompt {
        rmcp::model::Prompt {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            arguments: self.arguments.to_owned(),
        }
    }
}

impl Prompt {
    pub fn new() -> Self {
        let mut prompts = std::collections::HashMap::new();

        let prompt: std::sync::Arc<dyn IntoMcpPrompt> = std::sync::Arc::new(TemplatedPrompt {
            name: "git-commit-review".to_string(),
            description: Some("Review a git commit and suggest improvements.".to_string()),
            arguments: Some(vec![
                rmcp::model::PromptArgument {
                    name: "changes".to_owned(),
                    description: None,
                    required: Some(true),
                },
                rmcp::model::PromptArgument {
                    name: "commit_message".to_owned(),
                    description: None,
                    required: Some(true),
                },
            ]),
            template: r#"
I made the following changes:

{changes}

Based on these changes, I created a commit message in English. Please review this based on whether it's technically natural English for a Git commit message:

{commit_message}
"#.trim().to_owned(),
        });

        prompts.insert(prompt.name(), prompt);

        Self { prompts }
    }

    pub fn get_prompt(
        &self,
        request: rmcp::model::GetPromptRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::GetPromptResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            let rmcp::model::GetPromptRequestParam { name, arguments } = request;

            let result = self.prompts.get(&name);

            match result {
                Some(prompt) => Ok(prompt.into_get_result(arguments)),
                None => Err(rmcp::ErrorData::method_not_found::<
                    rmcp::model::GetPromptRequestMethod,
                >()),
            }
        }
    }

    pub fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListPromptsResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            let prompts = self
                .prompts
                .iter()
                .map(|(_, prompt)| prompt.into_list_result())
                .collect();

            Ok(rmcp::model::ListPromptsResult {
                next_cursor: None,
                prompts,
            })
        }
    }
}
