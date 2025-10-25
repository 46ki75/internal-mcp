use tokio::sync::OnceCell;

pub mod mcp_resources;
pub mod notion_api_key;

static SSM_CLIENT: OnceCell<aws_sdk_ssm::Client> = OnceCell::const_new();

async fn init_ssm_client() -> &'static aws_sdk_ssm::Client {
    SSM_CLIENT
        .get_or_init(|| async {
            let sdk_config = super::aws_config::init_sdk_config().await;

            let ssm_client = aws_sdk_ssm::Client::new(sdk_config);

            ssm_client
        })
        .await
}

pub async fn try_get_ssm_parameter_async(
    parameter_name: &str,
) -> Result<String, crate::error::Error> {
    let ssm_client = init_ssm_client().await;

    let parameter = ssm_client
        .get_parameter()
        .name(parameter_name)
        .with_decryption(true)
        .send()
        .await
        .map_err(|e| {
            let error = crate::error::Error::SsmParameterApiFailed {
                parameter_name: parameter_name.to_owned(),
                trace: e.to_string(),
            };
            tracing::error!("Failed to get parameter: {}", error);
            error
        })?
        .parameter
        .ok_or_else(|| {
            let error = crate::error::Error::SsmParameterNotFound {
                parameter_name: parameter_name.to_owned(),
            };
            tracing::error!("{}", error);
            error
        })?
        .value
        .ok_or_else(|| {
            let error = crate::error::Error::SsmParameterNotFound {
                parameter_name: parameter_name.to_owned(),
            };
            tracing::error!("{}", error);
            error
        })?;

    tracing::debug!("Fetching SSM Parameter: {}", parameter_name);

    Ok(parameter)
}
