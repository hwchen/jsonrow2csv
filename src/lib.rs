extern crate csv;
extern crate json;
#[macro_use]
pub extern crate slog;
extern crate slog_stdlog;

use csv::{Writer, QuoteStyle};
use json::JsonValue;
use json::object::Object;
use slog::{Drain, Logger};
use slog_stdlog::StdLog;

use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};

// Perf note: naive is 4.18s reading from file, 4.66 from stdin (out to dev/null)
// About 200MiB/s

pub fn json_to_csv<R: Read, W: Write>(
    reader: R,
    writer: W,
    keys: &[&str],
    logger: Option<Logger>,
    )
{
    let logger = logger.unwrap_or(Logger::root(StdLog.fuse(), o!()));
    let reader = BufReader::new(reader);

    let mut csv_writer = Writer::from_writer(writer)
        .escape(b'\\')
        .quote(b'\'')
        .quote_style(QuoteStyle::Necessary);


    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            match json::parse(&line) {
                Ok(parsed) => {
                    if let JsonValue::Object(object) = parsed {
                        let row = makerow(keys, &object);
                        csv_writer.encode(row).expect("problem writing csv");
                    }
                },
                Err(err) => {
                    error!(logger, "ParseError";
                        "line" => i,
                        "msg" => err.description(),
                    )
                },
            }
        }
    }
}

fn makerow<'a>(keys: &[&str], object: &'a Object) -> Vec<&'a str> {
    keys.iter()
        .map(|key| {
            match object.get(key).and_then(|val| val.as_str()) {
                Some(val) => val,
                None => "",
            }
        }).collect()
}

//keys = os.environ.get('KEYS').split(',')
//makerow = lambda obj: [obj[k] for k in keys]
//
//writer = csv.writer(sys.stdout, escapechar='\\', quoting=csv.QUOTE_MINIMAL, doublequote=False)
//
//for line in sys.stdin:
//    writer.writerow(makerow(json.loads(line)))
