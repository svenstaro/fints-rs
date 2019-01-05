use crate::messages::*;
use crate::se::to_string;

#[derive(Debug)]
pub struct Dialog {
    // TODO: Dedup this somehow.
    /// Bank code or "Bankleitzahl" (blz).
    pub bank_code: u32,

    /// Username or identification number.
    pub username: String,

    /// Pin or password.
    pub pin: String,

    /// The `customer_system_id` starts at `0` (as per C.8) and will be assigned by the bank on
    /// first sync.
    pub customer_system_id: String,

    /// The `message_no` starts at `1` and will be incremented for every message sent.
    pub message_no: u16,

    // pub dialogId: String,
    /// List of TAN methods as returned by the bank on first sync.
    pub tan_methods: Vec<String>,
    // pub hisalsVersion: u32,
    // pub hikazsVerson: u32,
}

impl Dialog {
    pub fn new(bank_code: u32, username: &str, pin: &str) -> Dialog {
        Dialog {
            // name: name.to_string(),
            bank_code,
            username: username.to_string(),
            pin: pin.to_string(),
            customer_system_id: "0".to_string(),
            message_no: 1,
            // dialogId: "0".to_string(),
            tan_methods: vec![],
            // hisalsVersion: 6,
            // hikazsVerson: 6,
        }
    }

    pub fn sync(&mut self) {
        let dialog_sync_message = DialogSyncMessage::new(self.bank_code, &self.username, &self.pin, &self.customer_system_id, self.message_no);
        // TODO Send request
        let serialized = to_string(&dialog_sync_message).unwrap();
        println!("{}", serialized);
        // this.systemId = response.systemId;
        // this.dialogId = response.dialogId;
        // this.hisalsVersion = response.segmentMaxVersion(HISALS);
        // this.hikazsVersion = response.segmentMaxVersion(HIKAZS);
        // this.tanMethods = response.supportedTanMethods;
        // await this.end();
    }

    pub fn init(&mut self) {
        // let dialog_init_message = DialogInitMessage::new(self.bank_code, &self.username, &self.pin, &self.customer_system_id, self.message_no);
        // // TODO Send request
        // let serialized = to_string(&dialog_sync_message).unwrap();
        // println!("{}", serialized);
    }
}
