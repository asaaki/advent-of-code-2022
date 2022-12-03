# [Advent Of Code 2022]

My solutions for each task.

Goals and Non-Goals are similar to last year:

## Goals

- Solve the puzzles.
- Solve them as quickly as possible.
- (optional) Refactor if something bothers me too much.
- Yet still: Have fun with the puzzles!
- Enjoy the coding practice and built up the "muscle memory" for Rust.
- Learn more Rust on the way.
- Become more confident.
- Allow rough edges here and there.
- Have fun!

## Semi-Goals

- Performance and efficiency.

  Usually try to optimize for less to no allocations if possible.
  (Maybe next year I'll try to do a `#![no_std]`/embedded setup.)

  Input data gets embedded to also avoid the filesystem. This can bloat the binaries though.

- Relatively idiomatic Rust

  Although not all common patterns will be used; enums are usually common, but AoC puzzles mostly lend themselves to simple string and number crunching. Sometimes shorter code might be more preferrable than the Rusty code.

## Non-Goals

- The most, best, optimal, idiomatic way of writing Rust.¹
- Perfectionism.
- Competition.

¹ <sup>_Well, a bit of idiomatic code is probably still good._</sup>

## Theme / Challenge

Multi-bin crate: Each day becomes its own binary, the CLI only needs switches for which part to run and if example inputs should be used.

## Setup

- multiple binaries, one for each day
- binary name must include day value
- prepare task: create files and fetch input (needs session cookie)
- each day: create bin file, compile, prepare run, code solution, puzzle run

<!-- links -->
[Advent Of Code 2022]: https://adventofcode.com/2022
