2020-01
=======

Solution for [Advent of Code 2020](https://adventofcode.com/2020) day 1.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 439
part2: 584
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [16.089 us 16.203 us 16.313 us]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

part2                   time:   [33.673 us 33.899 us 34.125 us]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```
