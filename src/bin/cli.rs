use fints::DialogInitMessage;
use fints::PinTanClient;

pub fn main() {
    let client = PinTanClient {
        url: "http://127.0.0.1:3000/cgi-bin/hbciservlet".to_string(),
        bank_code: 12345678,
        username: "test1".to_string(),
        pin: "1234".to_string(),
    };
    let accounts = client.get_accounts();
    println!("{:#?}", client);
    println!("{:#?}", accounts);
}
