use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotionMcpResourceDto {
    pub title: String,
    pub uri: String,
}
