#[macro_use]
extern crate clap;
extern crate csv;
extern crate json;

use std::env;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;

use clap::{Arg, App};
use csv::{QuoteStyle, WriterBuilder};
use json::JsonValue;

const KEYS_ENV_VAR: &'static str = "KEYS";

fn main() {
    let app = App::new("json2csv")
        .version(crate_version!())
        .author(crate_authors!())
        .about("converts lines of json to csv")
        .after_help("ADDITIONAL INFO: \n    \
            1) Errors logged to STDERR.\n\n    \
            2) This program can select keys from each row of JSON;\n       \
            use either -k option for each key, or specify all keys\n       \
            as comma-separated string in env var KEYS.")
        .arg(Arg::with_name("file_in")
             .value_name("FILE_IN")
             .help("file to read from. Default STDIN"))
        .arg(Arg::with_name("file_out")
             .value_name("FILE_OUT")
             .short("o")
             .takes_value(true)
             .help("output to file. Default STDOUT"))
        .arg(Arg::with_name("keys")
             .value_name("KEYS")
             .short("k")
             .long("keys")
             .takes_value(true)
             .multiple(true)
             .number_of_values(1)
             .help("for each row, filter by keys. Takes multiple values, one per -k"))
        .get_matches();

    let reader: Box<dyn Read> = match app.value_of("file_in") {
        Some(path) => Box::new(File::open(path).unwrap()),
        _ => Box::new(io::stdin()),
    };

    let writer: Box<dyn Write> = match app.value_of("file_out") {
        Some(path) => Box::new(File::open(path).unwrap()),
        _ => Box::new(io::stdout()),
    };

    let keys: Vec<_> = match app.values_of("keys") {
        Some(keys) => keys.map(|k|k.to_owned()).collect(),
        _ => {
            let env_keys = env::var(KEYS_ENV_VAR).unwrap();
            env_keys.split(',').map(|k| k.to_owned()).collect()
        }
    };

    json_to_csv(reader, writer, &keys);
}

pub fn json_to_csv<R: Read, W: Write>(
    reader: R,
    writer: W,
    keys: &[String],
    )
{
    let reader = BufReader::new(reader);

    let mut csv_writer = WriterBuilder::new()
        .escape(b'\\')
        .quote(b'\'')
        .quote_style(QuoteStyle::Necessary)
        .from_writer(writer);


    for line in reader.lines() {
        let line = line.unwrap();
        let parsed = json::parse(&line).unwrap();
        if let JsonValue::Object(obj) = parsed {
            for key in keys {
                let k = obj.get(key)
                    .map(|k| k.as_str().unwrap())
                    .unwrap_or("");
                csv_writer.write_field(k).unwrap();
            }
        }
    }
}
