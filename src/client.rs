use encoding_rs::ISO_8859_15;
use failure::Error;
use serde_derive::{Deserialize, Serialize};

use crate::dialog::Dialog;

#[derive(Debug, Serialize, Deserialize)]
pub struct SepaAccount;

/// The `PinTanClient` is the primary way to communicate with a bank.
#[derive(Debug, Serialize, Deserialize)]
pub struct PinTanClient {
    /// URL to the specific bank's PIN/TAN portal.
    pub url: String,

    /// Bank code or "Bankleitzahl" (blz).
    pub bank_code: u32,

    /// Username or identification number.
    pub username: String,

    /// Pin or password.
    pub pin: String,
}

impl PinTanClient {
    pub fn get_accounts(&self) -> Vec<SepaAccount> {
        let mut dialog = Dialog::new(self.bank_code, &self.username, &self.pin);
        self.sync(&dialog).unwrap();
        dialog.init();
        vec![SepaAccount]
    }

    pub fn sync(&self, dialog: &Dialog) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let msg = dialog.get_sync_message();
        let mut response = client.post(&self.url)
            .body(msg)
            .send()?;

        let bytes = base64::decode(&response.text()?)?;
        let (decoded, _, had_errors) = ISO_8859_15.decode(&bytes);

        println!("response {} {}", response.status(), decoded);
        Result::Ok(response.text()?)
    }
}
