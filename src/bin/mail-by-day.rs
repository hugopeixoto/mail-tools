extern crate chrono;

use itertools::Itertools;

use std::fs;
use std::io;

use chrono::{DateTime, NaiveDateTime, Utc};
use mailparse::*;
use std::io::prelude::*;

fn main() {
    let mut entries = fs::read_dir("messages")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    entries.sort();

    let dates = entries
        .iter()
        .map(|path| {
            let mut file = fs::File::open(path).unwrap();
            let mut contents = Vec::<u8>::new();
            file.read_to_end(&mut contents).unwrap();

            contents
        })
        .map(|contents| {
            parse_headers(&contents)
                .unwrap()
                .0
                .get_first_value("Date")
                .unwrap()
                .replace("Pacific Standard Time", "PST")
                .replace(" --", " -")
                .replace("-Nov-", " Nov ")
                .replace("Dom", "Sun")
                .replace("Dez", "Dec")
        })
        .map(|date| dateparse(&date).unwrap())
        .map(|ts| NaiveDateTime::from_timestamp(ts, 0))
        .map(|datetime| DateTime::<Utc>::from_utc(datetime, Utc).date())
        .group_by(|&date| date)
        .into_iter()
        .map(|(_date, v)| v.count())
        .max();

    println!("{}", entries.len());
    println!("{:?}", dates);
}
