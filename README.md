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

Day 3 was really embarassing. I didn't have time / focus to do anything well so 500 million loops it
is I guess.
