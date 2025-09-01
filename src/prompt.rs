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
Is this Git commit message written in technically natural English that native-speaking developers would find clear and appropriate? If there are any awkward points or possible improvements, please provide specific suggestions and reasons.
Purpose: I want to learn how to write commit messages that are accepted in English-speaking development environments.

## Commit Message: 

{}
        "#,
            commit_message
        ).trim().to_owned();

        let prompt = rmcp::model::PromptMessage {
            role: rmcp::model::PromptMessageRole::User,
            content: rmcp::model::PromptMessageContent::Text { text: text },
        };

        Ok(vec![prompt])
    }
}
