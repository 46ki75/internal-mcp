#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
struct ReviewGitCommitMessageParams {
    pub commit_message: String,
}

#[rmcp::prompt_router]
impl crate::handler::Handler {
    pub fn init_prompt_router() -> rmcp::handler::server::router::prompt::PromptRouter<Self> {
        Self::prompt_router()
    }

    #[rmcp::prompt(description = "Generate code based on requirements")]
    async fn review_git_commit_message(
        &self,
        rmcp::handler::server::wrapper::Parameters(ReviewGitCommitMessageParams{commit_message}) : rmcp::handler::server::wrapper::Parameters<ReviewGitCommitMessageParams>,
    ) -> Result<Vec<rmcp::model::PromptMessage>, rmcp::ErrorData> {
        let text = format!(
            r#"
        Git のコミットメッセージが技術的に自然な英語かどうかレビューしてください。

        ## Commit Message: 

        {}
        "#,
            commit_message
        );

        let prompt = rmcp::model::PromptMessage {
            role: rmcp::model::PromptMessageRole::User,
            content: rmcp::model::PromptMessageContent::Text { text: text },
        };

        Ok(vec![prompt])
    }
}
