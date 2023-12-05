# AOC 2023

[Advent of Code](https://adventofcode.com/) in Rust.

## Getting Started

You will need the [rust compiler](https://rustup.rs/) and the [just](https://github.com/casey/just)
command line task runner. (The command runner is optional but it's how I've structured things).

You can run the exercise for any given day by running `just day {num}`.

You can run the tests for any given day's _lib_ by running, e.g. `cargo t --lib day2`, or the _bin_
by running `cargo t --bin day_2`.

You can run benchmarks against any given day by running `just bench {num}`.

#### Notes

- day 1. easy once I realized I needed to use a two pointer approach instead of iterating from the
  beginning
- day 2. easy, mostly just string parsing. nom FTW
- day 3. yuck. Simple conceptually but I wasted a bunch of time trying to find clever abstractions
  for it and the code shows it -- its a mess of half-baked ideas, none of which are very good. the
  answer is right tho.
- day 4. the recursive solution was easy...there's probably a much faster DP solution but I have a
  day job
