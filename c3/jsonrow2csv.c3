module jsonrow2csv;
import std::core::string;
import std::io;
import bufio;
import json::lex;

fn int main(String[] args) {
	// TODO handle flags, multiple keys
	String key = args[1];

	WriteBuffer out;
	char[4096 * 8] out_buf;
	out.init(io::stdout(), &out_buf);

	File f = file::open(args[2], "r")!!;
	BufReader in;
	char[4096 * 8] in_buf;
	in.init(&f, &in_buf);

	char[4096] linebuf;
	while (true) @pool() {
		char[]? line = bufio::readline(&in, &linebuf);
		if (catch err = line) {
			if (err == io::EOF) {
				break;
			}
			return 1;
		}
		Lexer lexer;
		lexer.init((String)line);
		while (try tag = lexer.next()) {
			if (tag == FIELD_NAME && lexer.token_slice == key) {
				Tag val_tag = lexer.next()!!;
				// Only support unnested values right now.
				switch (val_tag) {
					case NULL:
					case TRUE:
					case FALSE:
					case STRING:
					case NUM: out.write(lexer.token_slice)!!;

					case ARRAY_START:
					case ARRAY_END:
					case OBJ_START:
					case OBJ_END:
					case FIELD_NAME: unreachable();
				}
			}
		}
		out.write_byte('\n')!!;
	};
	out.flush()!!;
	return 0;
}
