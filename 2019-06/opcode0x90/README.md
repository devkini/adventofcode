2019-06
=======

Solution for [Advent of Code 2019](https://adventofcode.com/2019) day 6.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 171213
part2: 292
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [4.0750 ms 4.1216 ms 4.1680 ms]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

part2                   time:   [112.71 us 114.25 us 115.89 us]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```
