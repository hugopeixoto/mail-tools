extern crate imap;
extern crate native_tls;

use mail_tools::message_date;

fn main() {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect(("imap.gmail.com", 993), "imap.gmail.com", &tls).unwrap();

    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let mut imap_session = client.login(username, password).unwrap();

    imap_session.select("[Gmail]/All Mail").unwrap();

    for idx in 0.. {
        let msg = if idx == 0 { 1 } else { idx * 1000 };

        if let Some(date) = message_date(msg, &mut imap_session) {
            println!("{} {}", msg, date.to_rfc3339());
        } else {
            break;
        }
    }
}
