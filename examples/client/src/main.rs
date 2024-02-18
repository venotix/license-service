use vlks::client::LicenseClientBootstrap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let license_client = LicenseClientBootstrap::new(22623).await;
    match license_client {
        Some(mut license_client) => {
            let response = &license_client.request_license("c97283f4-06ee-46c1-8e39-d337483f4eb1".to_string()).await.unwrap();
            println!("{:?}", response);
        }
        _ => {
            println!("Failed to connect to the server");
        }
    }

    Ok(())
}