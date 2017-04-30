#[macro_use]
extern crate clap;
extern crate jsonrow2csv;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs::File;
use std::sync::Arc;

use clap::{Arg, App};
use jsonrow2csv::json_to_csv;
use slog::{Drain, Logger};

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

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(Arc::new(drain), o!());

    let reader: Box<Read> = match app.value_of("file_in") {
        Some(path) => Box::new(File::open(path).unwrap()),
        _ => Box::new(io::stdin()),
    };

    let writer: Box<Write> = match app.value_of("file_out") {
        Some(path) => Box::new(File::open(path).unwrap()),
        _ => Box::new(io::stdout()),
    };

    // a little convoluted to work around two possible borrows.
    // Calls the main worker fn in either branch.
    match app.values_of("keys") {
        Some(keys) => {
            let keys: Vec<_> = keys.collect();
            json_to_csv(reader, writer, &keys, Some(logger));
        },
        _ => {
            let keys = env::var(KEYS_ENV_VAR) .expect("can't find env var");
            let keys: Vec<_> = keys.split(',').collect();
            json_to_csv(reader, writer, &keys, Some(logger));
        }
    };

}
