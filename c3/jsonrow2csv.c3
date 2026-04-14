module jsonrow2csv;
import std::collections::list;
import std::core::log;
import std::core::string;
import std::io;
import bufio;
import json::lex;
import opter;

fn int main(String[] args) {
	Opts opts;
	if (catch err = parse_cli(args, &opts)) {
		log::error("cli parse error: %s", err);
		return 1;
	}

	WriteBuffer out;
	char[4096 * 8] out_buf;
	out.init(io::stdout(), &out_buf);

	File f = file::open_path(opts.path, "r")!!;
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
			if (tag == FIELD_NAME && contains(opts.keys, lexer.token_slice)) {
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

struct Opts {
	String[] keys;
	Path path;
}

fn void? parse_cli(String[] args, Opts* opts, Allocator alloc = mem) {
	Opter opter;
	opter.init(args);
	@pool() {
		List{String} keys;
		keys.tinit();
		while (try arg = opter.next()) {
			if (arg.type == EOF) break;
			switch {
				case arg.is(SHORT, 'k'):
				case arg.is(LONG, "key"):
					keys.push(opter.value()!.as_str());
				case arg.is(VALUE):
					opts.path = arg.value.as_path(alloc)!;
				default: opter.err_unexpected_arg()!;
			}
		}
		opts.keys = keys.to_array(alloc);
	};
}

fn bool contains(String[] haystack, char[] needle) {
	foreach (item : haystack) {
		if (item == (String)needle) {
			return true;
		}
	}
	return false;
}
