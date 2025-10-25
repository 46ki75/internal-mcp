pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod once_cell_cache;
pub(crate) mod prompt;
pub(crate) mod resource;
pub(crate) mod tool;

pub fn stage_name() -> Result<String, crate::error::Error> {
    let stage_name = std::env::var("STAGE_NAME").map_err(|_| {
        let error = crate::error::Error::EnvironmentVariableNotFound {
            variable_name: "STAGE_NAME".to_owned(),
        };
        tracing::error!("{}", error);
        error
    })?;
    Ok(stage_name)
}

pub async fn boot() -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::ServiceExt;

    let handler = crate::handler::Handler::new()
        .serve(rmcp::transport::stdio())
        .await?;

    handler.waiting().await?;

    Ok(())
}
