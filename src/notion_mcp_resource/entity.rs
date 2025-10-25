use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NotionMcpResourceEntity {
    pub name: String,
    pub uri: String,
}
