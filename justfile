bench:
    cargo build --release && \
    werk -f c3/Werkfile -Dprofile=release && \
    hyperfine \
    './c3/target/jsonrow2csv gender test-large.jsonl' \
    './target/release/main -k gender test-large.jsonl'
