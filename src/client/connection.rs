use super::transport::Transport;
use crate::client::config::{ClientConfig, LoginCredentials};
use crate::client::operations::{ConnectionOperations, LoginError};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Connection<T: Transport, L: LoginCredentials> {
    pub incoming_messages: Arc<Mutex<T::Incoming>>,
    pub outgoing_messages: Arc<Mutex<T::Outgoing>>,
    pub channels: Mutex<HashSet<String>>,
    pub config: Arc<ClientConfig<L>>,
}

impl<T: Transport, L: LoginCredentials> Connection<T, L> {
    pub fn new(transport: T, config: Arc<ClientConfig<L>>) -> Connection<T, L> {
        // destructure the given transport...
        let (incoming_messages, outgoing_messages) = transport.split();

        // and build a Connection from the parts
        Connection {
            incoming_messages: Arc::new(Mutex::new(incoming_messages)),
            outgoing_messages: Arc::new(Mutex::new(outgoing_messages)),
            channels: Mutex::new(HashSet::new()),
            config,
        }
    }

    pub async fn initialize(&self) -> Result<(), LoginError<L::Error, T::OutgoingError>> {
        //let outgoing_messages = self.outgoing_messages.lock().await;
        // TODO this is a data race with other things also sending messages at connection startup
        //  we would ideally need a re-entrant mutex

        self.request_capabilities()
            .await
            .map_err(LoginError::TransportOutgoingError)?;
        self.login().await?;

        Ok(())
    }
}