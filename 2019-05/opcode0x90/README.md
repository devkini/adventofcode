2019-05
=======

Solution for [Advent of Code 2019](https://adventofcode.com/2019) day 5.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
part1: 12234644
part2: 3508186
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [412.87 ns 414.41 ns 416.00 ns]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

part2                   time:   [634.03 ns 637.55 ns 641.51 ns]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
```
