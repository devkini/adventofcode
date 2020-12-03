2020-03
=======

Solution for [Advent of Code 2020](https://adventofcode.com/2020) day 3.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 159
part2: 6419669520
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [8.6397 us 8.7360 us 8.8403 us]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

part2                   time:   [41.601 us 42.059 us 42.538 us]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
