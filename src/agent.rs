use log::debug;
use std::collections::HashMap;
use zbus::interface;
use zbus::zvariant::{ObjectPath, OwnedValue, Value};

pub struct PasswordAgent {
    passphrase: String,
}

impl PasswordAgent {
    pub fn new(passphrase: String) -> Self {
        Self { passphrase }
    }
}

#[interface(name = "net.connman.Agent")]
impl PasswordAgent {
    /// Called when ConnMan needs input from the user
    async fn request_input(
        &self,
        service: ObjectPath<'_>,
        fields: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<HashMap<String, Value<'static>>> {
        debug!("Agent: request_input called for service: {}", service);
        debug!("Agent: fields requested: {:?}", fields);

        let mut response = HashMap::new();

        // Check if Passphrase is requested
        if fields.contains_key("Passphrase") {
            debug!("Agent: Providing passphrase");
            response.insert(
                "Passphrase".to_string(),
                Value::new(self.passphrase.clone()),
            );
        }

        Ok(response)
    }

    /// Called to report an error
    async fn report_error(&self, service: ObjectPath<'_>, error: &str) -> zbus::fdo::Result<()> {
        debug!("Agent: Error reported for {}: {}", service, error);
        Ok(())
    }

    /// Called when user interaction is requested (browser, etc.)
    async fn request_browser(&self, service: ObjectPath<'_>, url: &str) -> zbus::fdo::Result<()> {
        debug!("Agent: Browser requested for {}: {}", service, url);
        Ok(())
    }

    /// Called when connection is released
    async fn release(&self) -> zbus::fdo::Result<()> {
        debug!("Agent: Released");
        Ok(())
    }

    /// Called when user needs to be notified
    async fn cancel(&self) -> zbus::fdo::Result<()> {
        debug!("Agent: Cancelled");
        Ok(())
    }
}

