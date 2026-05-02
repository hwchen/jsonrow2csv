# 2026-05-01

This project may be converted to `jrill`.

I came across `jsongrep` and wanted to know if I could be faster.
- https://github.com/micahkepe/jsongrep#why-jsongrep
- https://micahkepe.com/blog/jsongrep/

General takeaways:
- if anything, json parsing speed probably the limiter.
    - The most barebones one (jsonrow2csv) is the fastest.
    - fastgron (simdjson) is faster than jsongrep
- Maybe with more complicated json the search will take more time.
    - For example, I would guess that fastgron would no longer be able
      to beat fastgrep if fastgron had to generate bytes for many nesteds.
    - On flat structures, I would expect simple search on keys to be approx the same as jsongrep's DFA approach.
    - especially, note that a search strategy doesn't automatically do better
        - fastgron is rg, but is faster than jsongrep
        - picogron is rg, but is slower than jsongrep
        - search keys directly seems to do better, but the rust version isn't
            even much faster than fastgron generating paths.
        - So I believe it's just that jsonrow2csv is much simpler.

Goals for jrill:
- I want basic json to approach the best case of jsonrow2csv or at least the
    rs version.
- I want to control the whole stack, to make sure can optimize everything.
- I want to keep it super simple (no using deps that are more complicated
    because they're for more general use cases).
- For this kind of tool, I really am not sure if it needs to be a validating
    parser. It's more of a pass-through; people should be validating/parsing
    before using the data, but this util is not really "using" the data.
- Should I skip unicode GCD for printing idents? It should happen a lot less


```
~/projects/jsonrow2csv % just bench
cargo build --release && werk -f c3/Werkfile -Dprofile=release && hyperfine --warmup 3 './c3/target/jsonrow2csv -k gender test-large.jsonl' './target/release/main -k gender test-large.jsonl' 'jg -F gender test-large.jsonl' 'fastgron -s test-large.jsonl | rg gender' 'picogron -s test-large.jsonl | rg gender'
    Finished `release` profile [optimized] target(s) in 0.02s
[ ok ] build
Benchmark 1: ./c3/target/jsonrow2csv -k gender test-large.jsonl
  Time (mean ± σ):     376.3 ms ±   1.8 ms    [User: 259.3 ms, System: 116.9 ms]
  Range (min … max):   372.4 ms … 378.3 ms    10 runs
 
Benchmark 2: ./target/release/main -k gender test-large.jsonl
  Time (mean ± σ):      1.858 s ±  0.009 s    [User: 1.619 s, System: 0.239 s]
  Range (min … max):    1.849 s …  1.875 s    10 runs
 
Benchmark 3: jg -F gender test-large.jsonl
  Time (mean ± σ):      5.924 s ±  0.022 s    [User: 3.561 s, System: 2.362 s]
  Range (min … max):    5.884 s …  5.959 s    10 runs
 
Benchmark 4: fastgron -s test-large.jsonl | rg gender
  Time (mean ± σ):      2.893 s ±  0.017 s    [User: 1.390 s, System: 1.405 s]
  Range (min … max):    2.865 s …  2.912 s    10 runs
 
Benchmark 5: picogron -s test-large.jsonl | rg gender
  Time (mean ± σ):     15.579 s ±  0.037 s    [User: 7.644 s, System: 9.188 s]
  Range (min … max):   15.524 s … 15.662 s    10 runs
 
Summary
  ./c3/target/jsonrow2csv -k gender test-large.jsonl ran
    4.94 ± 0.03 times faster than ./target/release/main -k gender test-large.jsonl
    7.69 ± 0.06 times faster than fastgron -s test-large.jsonl | rg gender
   15.74 ± 0.10 times faster than jg -F gender test-large.jsonl
   41.40 ± 0.22 times faster than picogron -s test-large.jsonl | rg gender
```
