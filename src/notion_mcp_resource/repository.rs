use std::pin::Pin;

use futures::TryStreamExt;
use notionrs::PaginateExt;

use super::dto::*;
use crate::{
    notion_mcp_resource,
    once_cell_cache::{ssm_parameter::*, *},
};

use notionrs_types::prelude::*;

pub trait NotionRepository {
    fn list_resources(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<NotionMcpResourceDto>, crate::error::Error>>>>;

    fn get_resource(
        &self,
        page_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, crate::error::Error>>>>;
}

pub struct NotionRepositoryImpl {}

impl NotionRepository for NotionRepositoryImpl {
    fn list_resources(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<NotionMcpResourceDto>, crate::error::Error>>>> {
        Box::pin(async move {
            let notionrs_client = notionrs_client::init_notionrs_client().await?;

            let data_source_id = mcp_resources::init_mcp_resources_data_source_id().await?;

            let pages = notionrs_client
                .query_data_source()
                .data_source_id(data_source_id)
                .into_stream()
                .try_collect::<Vec<PageResponse>>()
                .await?;

            let mut resources = vec![];

            for page in pages {
                let property_name = "Name";

                let maybe_name = page.properties.get(property_name).ok_or(
                    crate::error::Error::NotionPagePropertyNotFound(property_name.to_owned()),
                )?;

                let title = if let PageProperty::Title(title) = maybe_name {
                    title
                        .title
                        .iter()
                        .map(|r| r.to_string())
                        .collect::<String>()
                } else {
                    return Err(crate::error::Error::NotionInvalidSchema("title".to_owned()));
                };

                resources.push(NotionMcpResourceDto {
                    title,
                    uri: format!("notion://{}", page.id),
                });
            }

            Ok(resources)
        })
    }

    fn get_resource(
        &self,
        page_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, crate::error::Error>>>> {
        let page_id = page_id.to_owned();

        Box::pin(async move {
            let notionrs_client = notionrs_client::init_notionrs_client().await?;

            let markdown = notionrs_client.to_markdown(page_id).await?.join("\n");

            Ok(markdown)
        })
    }
}
