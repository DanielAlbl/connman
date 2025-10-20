use crate::ManagerProxy;
use std::collections::HashMap;
use strum::AsRefStr;
use zbus::proxy;
use zbus::zvariant::{OwnedValue, Value};

/// ConnMan technology types
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum TechnologyType {
    Ethernet,
    WiFi,
    Bluetooth,
    Cellular,
    P2P,
    Gadget,
}

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

    pub async fn get_technology<'a>(
        connection: &'a zbus::Connection,
        manager: &ManagerProxy<'a>,
        technology_type: TechnologyType,
    ) -> zbus::Result<Option<TechnologyProxy<'a>>> {
        let technologies = manager.get_technologies().await?;
        let technology = technologies.into_iter().find(|(_, x)| {
            x.get("Type").and_then(|t| t.downcast_ref::<&str>().ok())
                == Some(technology_type.as_ref())
        });
        if let Some((path, _)) = technology {
            Ok(Some(
                TechnologyProxy::new_from_path(path, connection).await?,
            ))
        } else {
            Ok(None)
        }
    }
}
