use std::collections::HashMap;
use zbus::proxy;
use zbus::zvariant::{OwnedValue, Value};

#[proxy(interface = "net.connman.Technology", default_service = "net.connman")]
pub trait Technology {
    fn scan(&self) -> zbus::Result<()>;

    /// Get all Technology properties at once (bulk operation).
    /// This is the ONLY way to read properties in ConnMan - it doesn't support
    /// the standard D-Bus Properties interface.
    fn get_properties(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    /// Set a property by name (for dynamic use cases).
    /// Use this to set properties like "Powered", "Tethering", "TetheringIdentifier", etc.
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;
}

impl TechnologyProxy<'_> {
    pub async fn new_from_path(
        technology_path: zbus::zvariant::OwnedObjectPath,
        connection: &zbus::Connection,
    ) -> zbus::Result<TechnologyProxy<'_>> {
        TechnologyProxy::builder(connection)
            .path(technology_path)?
            .build()
            .await
    }
}
