use fints::DialogInitMessage;
use fints::PinTanClient;

pub fn main() {
    let client = PinTanClient {
        url: "https://127.0.0.1:3000/cgi-bin/hbciservlet".to_string(),
        bank_code: 12345678,
        username: "test".to_string(),
        pin: "12345".to_string(),
    };
    // let accounts = client.get_accounts();
    println!("{:#?}", client);
    // println!("{:#?}", accounts);
}
