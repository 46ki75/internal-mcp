static NOTIONRS_CLIENT: tokio::sync::OnceCell<notionrs::Client> =
    tokio::sync::OnceCell::const_new();

pub async fn init_notionrs_client() -> Result<&'static notionrs::Client, crate::error::Error> {
    NOTIONRS_CLIENT
        .get_or_try_init(|| async {
            let secret = super::ssm_parameter::notion_api_key::init_notion_api_key().await?;

            let client = notionrs::Client::new(secret.as_str());

            Ok(client)
        })
        .await
}
