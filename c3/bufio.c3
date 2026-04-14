module bufio;

import std::io;
import std::math;
import libc;

struct StdOut {
	File* stdout;
	char[] buffer;
	usz end;
}

fn void StdOut.init(&self, char[] buffer) {
	self.stdout = std::io::stdout();
	self.buffer = buffer;
}

fn usz? StdOut.writen(&self, char[] s) {
	if (self.end + s.len + 1 >= self.buffer.len) {
		self.stdout.write(self.buffer[0:self.end])!;
		self.end = 0;
	}
	mem::copy(&self.buffer[self.end], s.ptr, s.len);
	self.end += s.len;
	self.buffer[self.end] = '\n';
	self.end += 1;
	return s.len + 1;
}

fn void? StdOut.flush(&self) {
	self.stdout.write(self.buffer[0:self.end])!;
	self.end = 0;
}

struct BufReader {
	InStream wrapped_stream;
	char[] bytes;
	usz read_idx;
	usz write_idx;
}

fn void BufReader.init(&self, InStream stream, char[] buf) {
	self.wrapped_stream = stream;
	self.bytes = buf;
}

fn char? BufReader.read_byte(&self)
{
	if (self.read_idx == self.write_idx) {
		self.refill()!;
		if (self.read_idx == self.write_idx) return io::EOF~;
	}
	char c = self.bytes[self.read_idx];
	self.read_idx++;
	return c;
}

fn void? BufReader.refill(&self) @inline
{
	self.read_idx = 0;
	self.write_idx = self.wrapped_stream.read(self.bytes)!;
}

// advances the read_idx by n bytes.
// triggers a read if necessary.
// Should advance beyond write_idx, but only checks in debug.
fn void? BufReader.advance(&self, usz n) {
	self.read_idx += n;
	// leaving this in cause undefined behavior under release?
	//assert(self.read_idx > self.write_idx);
	if (self.read_idx == self.write_idx) {
		self.refill()!;
		if (self.read_idx == self.write_idx) return io::EOF~;
	}
}

fn char[] BufReader.buffer(&self) @inline {
	return self.bytes[self.read_idx..self.write_idx-1];
}


fn void? BufReader.close(&self) {
	return self.wrapped_stream.close();
}

// reads to a buf, instead of the std lib's reading to an allocation
// Now also uses memchr
// TODO double-check all the `advance` are correct
// TODO use BufReader buffer directly
macro char[]? readline(rdr = BufReader, char[] out_buf)
{
	rdr.advance(0)!;

	char[] buf = rdr.buffer();
	if (buf.len == 0) {
		return io::EOF~;
	}
	int out_buf_idx = 0;

	while (true) {
		void* end_label_ptr = libc::memchr(buf.ptr, '\n', buf.len);
		if (end_label_ptr == null) {
			// TODO buffer overflow
			out_buf[out_buf_idx:buf.len] = buf[..];
			out_buf_idx += buf.len;
			rdr.advance(buf.len)!!;
			buf = rdr.buffer();
		} else {
			int label_len = (int)(end_label_ptr - buf.ptr);
			// TODO buffer overflow
			out_buf[out_buf_idx:label_len] = buf[0:label_len];
			out_buf_idx += label_len;
			// ignore EOF here, handle on next readline
			if (catch err = rdr.advance((usz)(label_len + 1))) {
				if (err != io::EOF) {
					return err~;
				}
			}
			buf = rdr.buffer();
			break;
		}
	}
	return out_buf[:out_buf_idx];

}
