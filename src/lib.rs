extern crate csv;
extern crate json;

use csv::{Writer, QuoteStyle};
use json::JsonValue;
use json::object::Object;

use std::io::{BufRead, BufReader, Read, Write};

// Silently pass errors?
pub fn json_to_csv<R: Read, W: Write>(
    reader: R,
    writer: W,
    keys: &[&str],
    )
{
    let reader = BufReader::new(reader);

    let mut csv_writer = Writer::from_writer(writer)
        .escape(b'\\')
        .quote(b'\'')
        .quote_style(QuoteStyle::Necessary);


    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(parsed) = json::parse(&line) {
                if let JsonValue::Object(object) = parsed {
                    let row = makerow(keys, &object);
                    csv_writer.encode(row).expect("problem writing csv");
                }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

//keys = os.environ.get('KEYS').split(',')
//makerow = lambda obj: [obj[k] for k in keys]
//
//writer = csv.writer(sys.stdout, escapechar='\\', quoting=csv.QUOTE_MINIMAL, doublequote=False)
//
//for line in sys.stdin:
//    writer.writerow(makerow(json.loads(line)))
