extern crate json2csv;

use std::env;
use std::io;
use std::fs::File;

use json2csv::json_to_csv;

const KEYS_ENV_VAR: &'static str = "KEYS";

fn main() {
    let keys = env::var(KEYS_ENV_VAR).expect("can't find env var");
    let keys: Vec<_> = keys.split(',').collect();

    //let f = File::open("testfile-large.txt").unwrap();

    json_to_csv(io::stdin(), io::stdout(), &keys);
}
