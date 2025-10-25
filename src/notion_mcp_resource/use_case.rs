use crate::notion_mcp_resource::entity::NotionMcpResourceEntity;

use super::repository::NotionMcpResourceRepository;

#[derive(Clone)]
pub struct NotionMcpResourceUseCase {
    pub notion_mcp_resource_repository:
        std::sync::Arc<dyn NotionMcpResourceRepository + Send + Sync>,
}

impl NotionMcpResourceUseCase {
    pub async fn list_resources(
        &self,
    ) -> Result<Vec<NotionMcpResourceEntity>, Box<dyn std::error::Error>> {
        let dto = self.notion_mcp_resource_repository.list_resources().await?;

        let entities = dto
            .into_iter()
            .map(|dto| NotionMcpResourceEntity {
                name: dto.name,
                uri: dto.uri,
            })
            .collect::<Vec<NotionMcpResourceEntity>>();

        Ok(entities)
    }

    pub async fn get_resource(&self, uri: &str) -> Result<String, Box<dyn std::error::Error>> {
        let id = uri.strip_prefix("notion://").unwrap();

        let md = self.notion_mcp_resource_repository.get_resource(id).await?;

        Ok(md)
    }
}
