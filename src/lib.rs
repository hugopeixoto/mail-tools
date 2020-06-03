extern crate imap;
extern crate native_tls;
extern crate chrono;

use std::io::Write;

type ImapSession = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;
type DateTime = chrono::DateTime<chrono::offset::FixedOffset>;

#[derive(Debug)]
pub struct Message {
    pub uid: imap::types::Uid,
    pub body: Vec<u8>,
}

pub fn message_store(message: &Message) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create(format!("messages/{:07}.txt", message.uid))?;

    file.write_all(&message.body)?;

    Ok(())
}

pub fn message_body(n: u64, imap_session: &mut ImapSession) -> Option<Message> {
    if let Ok(messages) = imap_session.fetch(n.to_string(), "(UID RFC822)") {
        if let Some(message) = messages.first() {
            return Some(Message {
                uid: message.uid.unwrap(),
                body: message.body().map(|x| x.into_iter().map(|&x| x).collect()).unwrap(),
            });
        }
    }

    None
}

pub fn messages(from: u64, to: u64, imap_session: &mut ImapSession) -> Vec<Message> {
    if let Ok(messages) = imap_session.fetch(format!("{}:{}", from, to), "(UID RFC822)") {
        return messages.iter().map(|message|
            Message {
                uid: message.uid.unwrap(),
                body: message.body().map(|x| x.into_iter().map(|&x| x).collect()).unwrap(),
            }
        ).collect::<Vec<_>>();
    }

    vec![]
}

pub fn message_date(n: u64, imap_session: &mut ImapSession) -> Option<DateTime> {
    if let Ok(messages) = imap_session.fetch(n.to_string(), "INTERNALDATE") {
        if let Some(message) = messages.first() {
            return message.internal_date();
        }
    }

    None
}

pub fn highest_message_number(imap_session: &mut ImapSession) -> u64 {
    let base = 0;

    let mut lower = 0;
    let mut upper = 0;
    for e in 0.. {
        let idx = base + 4u64.pow(e);

        if let Some(_date) = message_date(idx, imap_session) {
            lower = idx;
        } else {
            upper = idx;
            break
        }
    }

    while lower + 1 != upper {
        let middle = (lower + upper) / 2;

        if let Some(_date) = message_date(middle, imap_session) {
            lower = middle;
        } else {
            upper = middle;
        }
    }

    lower
}

pub fn messages_store_from(base: u64, imap_session: &mut ImapSession) {
    let max = highest_message_number(imap_session);
    println!("Going up to {}", max);

    let step = 500;

    for i in 0.. {
        let lower = base + i * step;
        let upper = base + (i + 1) * step;

        if lower > max {
            break;
        }

        println!("batching {}:{}", lower, upper);
        for message in messages(lower, upper, imap_session).into_iter() {
            message_store(&message).unwrap();
        }
    }
}
