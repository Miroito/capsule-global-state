use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SomeExternalApi {
    endpoint: String,
}

impl SomeExternalApi {
    pub fn new(endpoint: String) -> Self {
        SomeExternalApi { endpoint }
    }

    pub async fn get_endpoint(&self) -> String {
        // The  engine implementation is here just for good measure because we never use it
        #[cfg(engine)]
        tokio::time::sleep(Duration::from_secs(1)).await;

        #[cfg(client)]
        async_std::task::sleep(Duration::from_secs(1)).await;

        self.endpoint.clone()
    }
}
