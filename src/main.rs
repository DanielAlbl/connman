use connman::ManagerProxy;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;
    let manager = ManagerProxy::new(&connection).await?;

    println!("ConnMan is running!");

    let services = manager.get_services().await?;
    println!("Found {} services", services.len());

    Ok(())
}
