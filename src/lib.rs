pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod prompt;
pub(crate) mod resource;
pub(crate) mod tool;

pub async fn boot() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::ServiceExt;

    let handler = crate::handler::Handler::new()
        .serve(rmcp::transport::stdio())
        .await?;

    handler.waiting().await?;

    Ok(())
}
