2019-03
=======

Solution for [Advent of Code 2019](https://adventofcode.com/2019) day 3.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
part1: 2193
part2: 63526
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1 --sample-size 20
part1                   time:   [34.457 ms 34.910 ms 35.446 ms]
Found 3 outliers among 20 measurements (15.00%)
  1 (5.00%) high mild
  2 (10.00%) high severe

part2                   time:   [35.793 ms 36.242 ms 36.800 ms]
Found 2 outliers among 20 measurements (10.00%)
  2 (10.00%) high severe
```
