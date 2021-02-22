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
part1: 969024
part2: 230057040
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [6.5370 us 6.5699 us 6.6047 us]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

Benchmarking part2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.3s, enable flat sampling, or reduce sample count to 60.
part2                   time:   [1.2364 ms 1.2439 ms 1.2538 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```
