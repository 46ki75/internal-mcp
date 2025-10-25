use tokio::sync::OnceCell;

static NOTION_API_KEY: OnceCell<String> = OnceCell::const_new();

pub async fn init_notion_api_key() -> Result<&'static String, crate::error::Error> {
    NOTION_API_KEY
        .get_or_try_init(|| async {
            let stage_name = crate::stage_name()?;

            let parameter = super::try_get_ssm_parameter_async(&format!(
                "/{stage_name}/46ki75/internal/notion/secret",
            ))
            .await?;

            Ok(parameter)
        })
        .await
}
