bench:
    cargo build --release && \
    werk -f c3/Werkfile -Dprofile=release && \
    hyperfine --warmup 3 \
    './c3/target/jsonrow2csv -k gender test-large.jsonl' \
    './target/release/main -k gender test-large.jsonl'

poop:
    cargo build --release && \
    werk -f c3/Werkfile -Dprofile=release && \
    poop \
    './c3/target/jsonrow2csv -k gender test-large.jsonl' \
    './target/release/main -k gender test-large.jsonl'

bench-multiple:
    cargo build --release && \
    werk -f c3/Werkfile -Dprofile=release && \
    hyperfine --warmup 100 \
    "./c3/target/jsonrow2csv -k index -k '_id' test.jsonl" \
    "./target/release/main -k index -k '_id' test.jsonl"

bench-big-multiple:
    cargo build --release && \
    werk -f c3/Werkfile -Dprofile=release && \
    hyperfine --warmup 10 \
    "./c3/target/jsonrow2csv -k index -k '_id' test-large.jsonl" \
    "./target/release/main -k index -k '_id' test-large.jsonl"
