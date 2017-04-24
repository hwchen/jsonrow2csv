## jsonrow2csv

A small utility to read rows of json and write to csv.

### Install

Make sure that Rust is installed. I suggest using [rustup.rs](https://rustup.rs).

This utility is developed on nightly, but uses no special features and should work
on stable Rust.

```
$ git clone https://github.com/hwchen/jsonrow2csv.git && cd jsonrow2csv
$ cargo install
```

This should install a binary executable in `~./cargo/bin`.

If you don't want to install, you can still run by doing:

```
$ cargo build --release && ./target/release/main
```

Dont forget `--release`, otherwise it will be slow!

### Input

Reads from `stdin` if no file specified, otherwise specify filepath as arg.

### Output

Writes to `stdout` if not file specified, otherwise specify filepath using
the `-o` option.

### Keys

You can specify which keys in each json row you want to filter for.

One method is to set an env var: `export KEYS=key1,key2,key3` with the keys
as one string separated by `,`.

Another method is to set by cli arg `-k`. Each `-k <key>` is one key; Because
of parsing concerns, multiple values cannot be used with one `-k`, but you can
specify several values using several `-k`.

### test file

The `test.json` file is a pseudo-random file containing 6 rows of json objects
(no nested objects or arrays).

I generated up to a 1gb test file using awk:

```
$ cat testfile-medium.txt | awk '{a[NR]=$0}END{for (i=0; i<10; i++){for(k in a){print a[k]}}}' > testfile-large.txt
```

This repeats each chunk of lines. Because I'm not an awk wizard, [google](https://askubuntu.com/questions/521465/how-can-i-repeat-the-content-of-a-file-n-times/521516#521516).

### Timings

Some quick timings:

```
mochi:json2csv (master|✔) > ls -l
total 1126440
drwxrwxr-x 3 hwchen hwchen       4096 Apr 21 23:21 src
drwxrwxr-x 4 hwchen hwchen       4096 Apr 23 13:29 target
-rw-rw-r-- 1 hwchen hwchen       6713 Apr 23 20:22 Cargo.lock
-rw-rw-r-- 1 hwchen hwchen        161 Apr 23 20:09 Cargo.toml
-rw-rw-r-- 1 hwchen hwchen 1047644040 Apr 23 13:59 testfile-large.txt
-rw-rw-r-- 1 hwchen hwchen  104764404 Apr 23 13:59 testfile-medium.txt
-rw-rw-r-- 1 hwchen hwchen    1027102 Apr 23 13:57 testfile-small.txt
-rw-rw-r-- 1 hwchen hwchen       5738 Apr 23 13:21 test.json
mochi:json2csv (master|✔) > cargo build --release && time cat testfile-large.txt | pv --rate | ./target/release/main > /dev/null
    Finished release [optimized] target(s) in 0.0 secs
    real 4.86s]
    user 0.00
    sys 0.56
    [ 205MiB/s]

```
