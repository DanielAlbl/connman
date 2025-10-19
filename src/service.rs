use std::collections::HashMap;
use zbus::proxy;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

#[proxy(interface = "net.connman.Service", default_service = "net.connman")]
pub trait Service {
    fn connect(&self) -> zbus::Result<()>;

    fn disconnect(&self) -> zbus::Result<()>;

    fn remove(&self) -> zbus::Result<()>;

    fn move_before(&self, service: &ObjectPath<'_>) -> zbus::Result<()>;

    fn move_after(&self, service: &ObjectPath<'_>) -> zbus::Result<()>;

    fn reset_counters(&self) -> zbus::Result<()>;

    /// Set a property by name (for dynamic use cases).
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;

    fn clear_property(&self, name: &str) -> zbus::Result<()>;

    /// Get all Service properties at once (bulk operation).
    /// This is the ONLY way to read properties in ConnMan - it doesn't support
    /// the standard D-Bus Properties interface.
    fn get_properties(&self) -> zbus::Result<HashMap<String, OwnedValue>>;
}

impl ServiceProxy<'_> {
    pub async fn new_from_path(
        service_path: OwnedObjectPath,
        connection: &zbus::Connection,
    ) -> zbus::Result<ServiceProxy<'_>> {
        ServiceProxy::builder(connection)
            .path(service_path)?
            .build()
            .await
    }
}
