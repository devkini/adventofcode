2020-02
=======

Solution for [Advent of Code 2020](https://adventofcode.com/2020) day 2.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 192
part2: 101
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [381.34 us 383.44 us 385.69 us]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

part2                   time:   [582.10 us 587.68 us 594.64 us]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe
```
