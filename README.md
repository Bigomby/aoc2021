# 🎄 Advent of Code 🎄

## What is this repo

This repository contains some solutions to the [Advent of Code](https://adventofcode.com) coding puzzles. I'll try to 
solve all the puzzles but I'll probably fail at some point 😪.

## Goals

- Make it fast but readable: try to make it as fast as possible while still leveraging Rust idioms and standard library.
- Use generics: reduce dynamic dispatching when you can. Don't worry about binary size, 
- Do not reinvent the wheel: use libraries like [itertools](https://github.com/rust-itertools/itertools) when possible.
- Correct error handling: don't `unwrap()` everything. Use [anyhow](https://github.com/dtolnay/anyhow).

## How to run

To solve all the puzzles you can run the following command:

```
cargo test
```

This will use the Rust test runner to execute all the puzzles. It will run both examples and real input. 
If you want to test your own input you need to replace the inputs files in the `input` folder and change
the expected result in each puzzle test:

```
src/puzzles/puzzle_YYYY_XX/mod.rs
```