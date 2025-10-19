use std::collections::HashMap;
use zbus::{Connection, proxy};
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

// Define the ConnMan Manager interface
#[proxy(
    interface = "net.connman.Manager",
    default_service = "net.connman",
    default_path = "/"
)]
trait Manager {
    fn get_properties(&self) -> zbus::Result<HashMap<String, OwnedValue>>;
    fn get_services(&self) -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;
    fn get_technologies(&self) -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;
}

// Define the ConnMan Technology interface (WiFi, Ethernet, Bluetooth, etc.)
#[proxy(
    interface = "net.connman.Technology",
    default_service = "net.connman"
)]
trait Technology {
    fn scan(&self) -> zbus::Result<()>;
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;
}

// Define the ConnMan Service interface (individual network connections)
#[proxy(
    interface = "net.connman.Service",
    default_service = "net.connman"
)]
trait Service {
    fn connect(&self) -> zbus::Result<()>;
    fn disconnect(&self) -> zbus::Result<()>;
    fn remove(&self) -> zbus::Result<()>;
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;
    fn clear_property(&self, name: &str) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the system bus
    let connection = Connection::system().await?;

    // Create a proxy to the ConnMan Manager interface
    let manager = ManagerProxy::new(&connection).await?;

    println!("Connected to ConnMan!\n");

    // Get and display manager properties
    println!("=== Manager Properties ===");
    let properties = manager.get_properties().await?;
    for (key, value) in properties {
        println!("{}: {:?}", key, value);
    }

    // Get and display available technologies
    println!("\n=== Technologies ===");
    let technologies = manager.get_technologies().await?;
    for (path, props) in technologies {
        println!("\nTechnology: {}", path);
        for (key, value) in props {
            println!("  {}: {:?}", key, value);
        }
    }

    // Get and display available services
    println!("\n=== Services ===");
    let services = manager.get_services().await?;
    for (path, props) in services {
        println!("\nService: {}", path);
        for (key, value) in props {
            println!("  {}: {:?}", key, value);
        }
    }

    Ok(())
}
