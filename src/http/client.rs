use std::num::NonZeroU64;

use reqwest::Client;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Message {}

#[derive(Debug, Clone, Copy)]
pub enum HttpError {}

pub struct Http {
    pub client: Client,
    token: SecretString,
}

impl Http {
    pub async fn send_message(
        self,
        channel_id: NonZeroU64,
        content: String,
    ) -> Result<Message, HttpError> {
        todo!()
    }
}
