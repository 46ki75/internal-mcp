#[tokio::main]
async fn main() {
    let _ = internal_mcp::boot().await;
}
