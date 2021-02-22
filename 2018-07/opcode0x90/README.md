2018-07
=======

Solution for [Advent of Code](https://adventofcode.com/2018) day 7.

Compiling and Running
---------------------

1. Install [Rust](https://www.rust-lang.org/en-US/install.html).
2. `cargo run --release`

Result
------

```sh
part1: GDHOSUXACIMRTPWNYJLEQFVZBK
part2: 1024
```

Benchmark
---------

```
$ cargo bench -- --save-baseline 1
part1                   time:   [7.3964 us 7.5470 us 7.7233 us]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

part2                   time:   [14.591 us 14.719 us 14.866 us]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
```
