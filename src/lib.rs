extern crate csv;
extern crate json;

use csv::{WriterBuilder, QuoteStyle};
use json::JsonValue;

use std::io::{BufRead, BufReader, Read, Write};

// Perf note: naive is 4.18s reading from file, 4.66 from stdin (out to dev/null)
// About 200MiB/s

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
