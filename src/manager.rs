use std::collections::HashMap;
use zbus::proxy;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

#[proxy(
    interface = "net.connman.Manager",
    default_service = "net.connman",
    default_path = "/"
)]
pub trait Manager {
    /// Get all Manager properties at once (bulk operation).
    /// This is the ONLY way to read properties in ConnMan - it doesn't support
    /// the standard D-Bus Properties interface.
    fn get_properties(&self) -> zbus::Result<HashMap<String, OwnedValue>>;

    fn get_services(&self) -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;

    fn get_technologies(&self)
    -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;

    fn register_agent(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    fn unregister_agent(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Set a property by name (for dynamic use cases).
    /// Use this to set properties like "OfflineMode" or "SessionMode".
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;

    // Signals:
    #[zbus(signal)]
    fn technology_added(
        &self,
        path: OwnedObjectPath,
        properties: HashMap<String, OwnedValue>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    fn technology_removed(&self, path: OwnedObjectPath) -> zbus::Result<()>;

    #[zbus(signal)]
    fn services_changed(
        &self,
        changed: Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>,
        removed: Vec<OwnedObjectPath>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    fn peers_changed(
        &self,
        changed: Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>,
        removed: Vec<OwnedObjectPath>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    fn tethering_clients_changed(
        &self,
        registered: Vec<String>,
        removed: Vec<String>,
    ) -> zbus::Result<()>;
}
