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
part1: 460
part2: 290
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1 --sample-size 20
part1                   time:   [45.484 ms 45.709 ms 45.925 ms]
Found 3 outliers among 20 measurements (15.00%)
  2 (10.00%) high mild
  1 (5.00%) high severe

Benchmarking part2: Warming up for 3.0000 s
part2                   time:   [75.470 ms 75.959 ms 76.516 ms]
Found 1 outliers among 20 measurements (5.00%)
  1 (5.00%) high severe
```
