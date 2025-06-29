### WIP cargo build benches

```
$ cargo run --example macro_heavy
entries: 1342

$ cargo run --example no_macro
entries: 1342
```

macro heavy
```
$ hyperfine --prepare 'cargo clean' 'cargo build --release --example macro_heavy'
Benchmark 1: cargo build --release --example macro_heavy
  Time (mean ± σ):      3.029 s ±  0.017 s    [User: 2.988 s, System: 0.211 s]
  Range (min … max):    2.986 s …  3.042 s    10 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
```

no macro
```
$ hyperfine --prepare 'cargo clean' 'cargo build --release --example no_macro'
Benchmark 1: cargo build --release --example no_macro
  Time (mean ± σ):      5.409 s ±  0.032 s    [User: 5.301 s, System: 0.273 s]
  Range (min … max):    5.374 s …  5.465 s    10 runs
```

observation: heavy use of macro's is faster than no macro

try:
- panic abort
- get rid of .unwraps

no unwraps at all:

```
$ hyperfine --prepare 'cargo clean' 'cargo build --release --example no_macro3'
Benchmark 1: cargo build --release --example no_macro3
  Time (mean ± σ):      1.583 s ±  0.007 s    [User: 1.575 s, System: 0.181 s]
  Range (min … max):    1.575 s …  1.595 s    10 runs
```
