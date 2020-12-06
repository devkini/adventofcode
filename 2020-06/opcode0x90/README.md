2020-06
=======

Solution for [Advent of Code 2020](https://adventofcode.com/2020) day 6.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 6297
part2: 3158
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [809.99 us 813.64 us 817.36 us]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking part2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.9s, enable flat sampling, or reduce sample count to 60.
part2                   time:   [1.0969 ms 1.1021 ms 1.1075 ms]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
```
