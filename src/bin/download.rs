extern crate imap;
extern crate native_tls;
extern crate chrono;

use mail_tools::{*};

fn main() {
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect(
        ("imap.gmail.com", 993),
        "imap.gmail.com",
        &tls,
    ).unwrap();

    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let mut imap_session = client
      .login(username, password)
      .unwrap();

    imap_session.select("[Gmail]/All Mail").unwrap();

    messages_store_from(1, &mut imap_session);
}
