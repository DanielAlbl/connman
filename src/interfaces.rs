use std::collections::HashMap;
use zbus::proxy;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

#[proxy(
    interface = "net.connman.Manager",
    default_service = "net.connman",
    default_path = "/"
)]
pub trait Manager {
    fn get_properties(&self) -> zbus::Result<HashMap<String, OwnedValue>>;
    fn get_services(&self) -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;
    fn get_technologies(&self)
    -> zbus::Result<Vec<(OwnedObjectPath, HashMap<String, OwnedValue>)>>;
}

#[proxy(interface = "net.connman.Technology", default_service = "net.connman")]
pub trait Technology {
    fn scan(&self) -> zbus::Result<()>;
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;
}

#[proxy(interface = "net.connman.Service", default_service = "net.connman")]
pub trait Service {
    fn connect(&self) -> zbus::Result<()>;
    fn disconnect(&self) -> zbus::Result<()>;
    fn remove(&self) -> zbus::Result<()>;
    fn set_property(&self, name: &str, value: &Value<'_>) -> zbus::Result<()>;
    fn clear_property(&self, name: &str) -> zbus::Result<()>;
}
