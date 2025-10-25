static SDK_CONFIG: tokio::sync::OnceCell<aws_config::SdkConfig> =
    tokio::sync::OnceCell::const_new();

pub async fn init_sdk_config() -> &'static aws_config::SdkConfig {
    SDK_CONFIG
        .get_or_init(|| async {
            aws_config::defaults(aws_config::BehaviorVersion::latest())
                .load()
                .await
        })
        .await
}
