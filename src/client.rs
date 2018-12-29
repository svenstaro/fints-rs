use reqwest::get;

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
        dialog.sync();
        dialog.init();
        vec![SepaAccount]
    }
}
