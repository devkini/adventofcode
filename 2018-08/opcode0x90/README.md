2018-08
=======

Solution for [Advent of Code](https://adventofcode.com/2018) day 8.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 46781
part2: 21405
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [19.429 us 19.913 us 20.471 us]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

part2                   time:   [5.1786 us 5.2295 us 5.2837 us]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```
