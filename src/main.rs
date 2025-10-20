use anyhow::Result;
use connman::{ManagerProxy, PasswordAgent, ServiceProxy, TechnologyProxy, TechnologyType};
use zbus::Connection;
use zbus::zvariant::ObjectPath;

async fn connect_to_wifi(connection: &Connection, ssid: &str, password: &str) -> Result<()> {
    let manager = ManagerProxy::new(connection).await?;

    // Register our agent to handle authentication
    let agent_path = ObjectPath::try_from("/com/example/connman_agent")?;
    let agent = PasswordAgent::new(password.to_string());

    // Serve the agent on the connection
    connection.object_server().at(&agent_path, agent).await?;

    println!("Registering agent...");
    manager.register_agent(&agent_path).await?;

    // Get all services
    let services = manager.get_services().await?;

    // Find the service matching the SSID
    let some_ssid = Some(ssid.to_string());
    for (service_path, properties) in services {
        if properties.get("Name").map(|name| name.to_string()) == some_ssid {
            println!("Found network: {}", ssid);

            // Create a service proxy for this network
            let service = ServiceProxy::new_from_path(service_path, connection).await?;

            // Remove any previous configuration to ensure fresh connection
            println!("Removing any previous configuration...");
            let _ = service.remove().await; // Ignore errors if not configured yet

            println!("Connecting to {}...", ssid);
            service.connect().await?;
            println!("Successfully connected to {}", ssid);

            // Unregister the agent
            manager.unregister_agent(&agent_path).await?;
            println!("Agent unregistered");

            return Ok(());
        }
    }

    // Unregister agent if we didn't find the network
    manager.unregister_agent(&agent_path).await?;
    anyhow::bail!("Network '{}' not found. Make sure it's in range.", ssid)
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = &Connection::system().await?;
    let manager = &ManagerProxy::new(connection).await?;
    let wifi = TechnologyProxy::get_technology(connection, manager, TechnologyType::WiFi)
        .await?
        .unwrap();
    wifi.scan().await?;
    println!("ConnMan is running!");

    Ok(())
}
