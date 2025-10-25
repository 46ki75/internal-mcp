#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("EnvironmentVariableNotFound: environment variable not found: `{variable_name}`")]
    EnvironmentVariableNotFound { variable_name: String },

    #[error("SsmParameterApiFailed: failed to fetch parameter: `{parameter_name}`, trace: {trace}")]
    SsmParameterApiFailed {
        parameter_name: String,
        trace: String,
    },

    #[error("SsmParameterNotFound: parameter not found: `{parameter_name}`")]
    SsmParameterNotFound { parameter_name: String },

    #[error("Notion API request failed: {0}")]
    NotionApi(#[from] notionrs::Error),

    #[error("property '{0}' not found in Notion page")]
    NotionPagePropertyNotFound(String),

    #[error("property '{0}' has unexpected schema type")]
    NotionInvalidSchema(String),
}
