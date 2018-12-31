2018-09
=======

Solution for [Advent of Code](https://adventofcode.com/2018) day 9.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
$ cargo run --release
part1: 375465
part2: 3037741441
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [510.64 us 515.75 us 521.41 us]
                        change: [-3.2435% -0.0410% +3.0392%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

part2                   time:   [88.518 ms 88.755 ms 88.990 ms]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
```
