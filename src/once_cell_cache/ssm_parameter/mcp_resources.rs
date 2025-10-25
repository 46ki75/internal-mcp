use tokio::sync::OnceCell;

static MCP_RESOURCES_DATA_SOURCE_ID: OnceCell<String> = OnceCell::const_new();

pub async fn init_mcp_resources_data_source_id() -> Result<&'static String, crate::error::Error> {
    MCP_RESOURCES_DATA_SOURCE_ID
        .get_or_try_init(|| async {
            let stage_name = crate::stage_name()?;

            let parameter = super::try_get_ssm_parameter_async(&format!(
                "/{stage_name}/46ki75/internal/notion/mcp_resources/data_source/id",
            ))
            .await?;

            Ok(parameter)
        })
        .await
}
