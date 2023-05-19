use aws_sdk_s3 as s3;

#[tokio::main]
async fn main() -> Result<(), s3::Error> {
    let config = aws_config::from_env()
        .endpoint_url("http://localhost:9090")
        .load()
        .await;
    let client = s3::Client::new(&config);
    let result = client.list_buckets().send().await?;
    println!("Buckets: {:?}", result);
    Ok(())
}
