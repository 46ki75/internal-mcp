use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotionMcpResourceDto {
    pub name: String,
    pub uri: String,
}
