use std::num::NonZeroU64;

use reqwest::Client;
use secrecy::SecretString;

pub struct Http {
    pub client: Client,
    token: SecretString,
}

impl Http {
    pub async fn send_message(self, channel_id: NonZeroU64,) {}
}
