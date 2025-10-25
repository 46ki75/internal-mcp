use crate::notion_mcp_resource::{
    repository::NotionMcpResourceRepositoryImpl, use_case::NotionMcpResourceUseCase,
};

#[derive(Clone)]
pub struct Resource {
    pub notion_mcp_resource_use_case: NotionMcpResourceUseCase,
}

impl Resource {
    pub fn new() -> Self {
        let notion_mcp_resource_repository = NotionMcpResourceRepositoryImpl {};
        let notion_mcp_resource_use_case = NotionMcpResourceUseCase {
            notion_mcp_resource_repository: std::sync::Arc::new(notion_mcp_resource_repository),
        };

        Self {
            notion_mcp_resource_use_case,
        }
    }

    pub fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListResourcesResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            let notion_resources = self
                .notion_mcp_resource_use_case
                .list_resources()
                .await
                .unwrap();

            let resources = notion_resources
                .iter()
                .map(|resource| {
                    rmcp::model::Resource::new(
                        rmcp::model::RawResource {
                            uri: resource.uri.clone(),
                            name: resource.name.clone(),
                            description: None,
                            mime_type: None,
                            size: None,
                        },
                        None,
                    )
                })
                .collect::<Vec<rmcp::model::Annotated<rmcp::model::RawResource>>>();

            Ok(rmcp::model::ListResourcesResult {
                next_cursor: None,
                resources,
            })
        }
    }

    pub fn read_resource(
        &self,
        request: rmcp::model::ReadResourceRequestParam,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ReadResourceResult, rmcp::ErrorData>> + Send + '_
    {
        async {
            let uri = request.uri;

            let resource = self.notion_mcp_resource_use_case.get_resource(&uri).await;

            let contents = match resource {
                Ok(resource) => {
                    let content = rmcp::model::ResourceContents::TextResourceContents {
                        uri: uri.clone(),
                        mime_type: None,
                        text: resource,
                    };
                    Ok(rmcp::model::ReadResourceResult {
                        contents: vec![content],
                    })
                }
                Err(_) => Err(rmcp::ErrorData::resource_not_found(
                    format!("Resource not found: {}", uri),
                    None,
                )),
            };
            contents
        }
    }

    pub fn list_resource_templates(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<rmcp::model::ListResourceTemplatesResult, rmcp::ErrorData>>
    + Send
    + '_ {
        async {
            // let template = rmcp::model::RawResourceTemplate {
            //     name: "test".to_owned(),
            //     description: None,
            //     uri_template: "test:///yeah/{name}".to_owned(),
            //     mime_type: Some("text/plain".to_owned()),
            // };

            // let annotated = rmcp::model::Annotated {
            //     annotations: None,
            //     raw: template,
            // };

            let result = rmcp::model::ListResourceTemplatesResult {
                next_cursor: None,
                resource_templates: vec![],
            };

            Ok(result)
        }
    }
}
