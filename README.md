# Compiler bug

This compiler bug can be reproduced on:

```bash
rustc 1.39.0 (4560ea788 2019-11-04)
binary: rustc
commit-hash: 4560ea788cb760f0a34127156c78e2552949f734
commit-date: 2019-11-04
host: x86_64-unknown-linux-gnu
release: 1.39.0
LLVM version: 9.0
```

## Files

You can find compiler error in file `main.rs`.
Simplified version of error in file `src/main_simplified.rs` (error not faced/not working).

## Error

To reproduce error run `cargo build` or `cargo run`, after that you will see following error:

```bash
$ cargo build
   Compiling compiler-bug v0.1.0 (/dev/compiler-bug)
thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1165:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0 (4560ea788 2019-11-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: could not compile `compiler-bug`.

To learn more, run the command again with --verbose.
```
