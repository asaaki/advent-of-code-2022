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

## Solve times and benchmarking

Each day prints its solution in the following format:

```
[D01.1] solution: 69177
        solved in 33.63µs
```

Single day helper:

```sh
# fish
function day; day$argv; day$argv -s; end

# and call it
day 5
```

To get a list of all days and parts:

```sh
### fish ###

for d in (seq 1 5); day$d; day$d -s; end

# as a self deleting function with input last
function aoc; for d in (seq 1 $argv); day$d; day$d -s; end; functions -e aoc; end; aoc 5

# or if you want to reuse it
function aoc; for d in (seq 1 $argv); day$d; day$d -s; end; end
aoc 5 # call this in your current shell session

### bash ###

for d in $(seq 1 5); do day$d; day$d -s; done

# as a self deleting function with input last
aoc() { for d in $(seq 1 $1); do day$d; day$d -s; done; unset -f aoc; }; aoc 5

# or if you want to reuse it
aoc() { for d in $(seq 1 $1); do day$d; day$d -s; done }
aoc 5 # call this in your current shell session
```

To get a binary runtime comparison with [hyperfine]:

```sh
hyperfine -N -w 100 "day{day}" "day{day} -s" -P day 1 5
```

<!-- links -->
[Advent Of Code 2022]: https://adventofcode.com/2022
[hyperfine]: https://github.com/sharkdp/hyperfine
