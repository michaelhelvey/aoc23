# AOC 2023

[Advent of Code](https://adventofcode.com/) in Rust.

## Getting Started

You will need the [rust compiler](https://rustup.rs/) and the [just](https://github.com/casey/just)
command line task runner. (The command runner is optional but it's how I've structured things).

You can run the exercise for any given day by running `just day {num}`.

You can run the tests for any given day's _lib_ by running, e.g. `cargo t --lib day2`, or the _bin_
by running `cargo t --bin day_2`.

You can run benchmarks against any given day by running `just bench {num}`.

#### What on earth is going on with the file structure?

idk man. I wanted to have a binary for each day but not be restricted to writing all the code for
every day in a single file. The naming convention is super fucked though I agree, I'm just
"embracing the choas" (iykyk).
