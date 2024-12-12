# Billion Row Challenge <!-- omit in toc -->

> This challenge is inspired by the original [Java version](https://1brc.dev/), including the rules,
> dataset and task. Thank you Gunnar Morling for coming up with the
> [original idea](https://www.morling.dev/blog/one-billion-row-challenge/)!

This repository contains a template and different solutions for the Billion Row Challenge we ran for
the Rust Munich meetup in December 2024. We tried to keep deviations from the original challenge to
a minimum for the sake of comparability, but since the event takes place in a single evening there
are slight adaptations.

## Table of Contents <!-- omit in toc -->

- [Challenge](#challenge)
  - [Task Description](#task-description)
  - [Rules and Limitations](#rules-and-limitations)
- [Project Particularities](#project-particularities)
  - [Getting Started](#getting-started)
  - [How To Submit](#how-to-submit)
  - [Helpful Commands](#helpful-commands)
  - [Tips \& Tricks](#tips--tricks)
- [Advanced Techniques](#advanced-techniques)
- [Setup Issues?](#setup-issues)
- [Additional Resources](#additional-resources)

## Challenge

### Task Description

[original source](https://1brc.dev/#%F0%9F%92%AA-the-challenge)

Your mission, should you choose to accept it, is to write a program that retrieves temperature
measurement values from a text file and calculates the min, mean, and max temperature per weather
station. There's just one caveat: the file has 1,000,000,000 rows! That's more than 10 GB of data! 😱

The text file has a simple structure with one measurement value per row:

```txt
Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
Hamburg;34.2
St. John's;15.2
Cracow;12.6
... etc. ...
```

The program should print out the min, mean, and max values per station, alphabetically ordered. The
format that is expected varies slightly from language to language, but the following example shows
the expected output for the first three stations:

```txt
{
  Abha=5.0/18.0/27.4,
  Abidjan=15.7/26.0/34.1,
  Abéché=12.1/29.4/35.6,
  Accra=14.7/26.4/33.1,
  Addis Ababa=2.1/16.0/24.3,
  Adelaide=4.1/17.3/29.7,
  <...>
}
```

Oh, and this input.txt is different for each submission since it's generated on-demand. So no
hard-coding the results! 😉

### Rules and Limitations

[original source](https://1brc.dev/#rules-and-limits)

1. You may use external libraries (like optimized data structures, parsers, etc.) with the exception
   of libraries that can solve the full challenge by themselves (e.g. polars, embedded DuckDB, etc.)
   _This rule deviates from the original challenge due to the short duration of the event._
2. Implementations must implemented in main.rs/utils.rs only. Try to keep it relatively short; don't
   copy-paste a forbidden library into your solution as a cheat.
3. The computation must happen at application runtime; you cannot process the measurements file at
   build time.
4. Input value ranges are as follows:
   - Station name: non null UTF-8 string of min length 1 character and max length 100 bytes (i.e.
     this could be 100 one-byte characters, or 50 two-byte characters, etc.)
   - Temperature value: non-null double between -99.9 (inclusive) and 99.9 (inclusive), always with
     exactly one fractional digit
5. There is a maximum of 10,000 unique station names.
6. Implementations must not rely on specifics of a given data set. Any valid station name as per the
   constraints above and any data distribution (number of measurements per station) must be
   supported.

## Project Particularities

### Getting Started

> Note: running `export GITHUB_HANDLE=<your-github-handle>` should make these instructions easier.

Please run the following commands:

```sh
git clone git@github.com:rust-meetup-munich/hack-evening-2024-4.git
cd hack-evening-2024-4
git checkout -b solutions/${GITHUB_HANDLE}
cargo new solutions/${GITHUB_HANDLE}
```

The commands above will create a new git branch and initialize a new binary Rust project for your
solution. It will have the following simple structure; we expect your solution to be implemented in
the `main.rs` file.

```text
solutions/your-handle/
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
```

### How To Submit

-> Check out the [submission guide](https://rust-meetup-munich.github.io/hack-evening-2024-4/guide/src/02_submission.html) for more details.

### Helpful Commands

| Task                     | Folder                     | Command                                          |
| ------------------------ | -------------------------- | ------------------------------------------------ |
| generate 1B rows of data | data-generator             | `cargo run --release -- --rows 1000000000`       |
| run a debug build        | solutions/${GITHUB_HANDLE} | `cargo run           -- weather_1M.csv`          |
| run a release build      | solutions/${GITHUB_HANDLE} | `cargo run --release -- weather_1B.csv`          |
| run tests                | solutions/${GITHUB_HANDLE} | `cargo test`                                     |
| run flamegraph           | solutions/${GITHUB_HANDLE} | `sudo cargo flamegraph --root -- weather_1B.csv` |

### Tips & Tricks

- This event is rather short, so we recommend going for a functionally correct, simple solution
  first and then iteratively improve parts.
- When developing, use debug builds and smaller datasets (e.g. 10M rows) to avoid long computation
  times.
- Keep your intermediate solutions by committing them, both for discussions and comparisons.
- Use a profiler like [flamegraph](https://github.com/flamegraph-rs/flamegraph) to find out the
  actual bottlenecks in your program.

## Advanced Techniques

This section lists a few advanced techniques you may want to consider:

- use preallocations and avoid additional ones
- implement specialized float parsing
- custom hash functions
- concurrency
  - reading the file with multiple threads in parallel
  - concurrent hashmaps like dash
  - SIMD instructions for parsing
- I/O techniques
  - buffered I/O
  - memory mapped files
  - asynchronous I/O with io_uring

## Setup Issues?

In case you have setup issues and cannot program locally, we prepared a Rust playground template
to give you a chance to still develop an algorithmic idea and dip your toe into Rust 🦀. Proper I/O,
large input data and extensive benchmarking are not possible, so a proper submission will be tricky.

[Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d039ad90338c5d71533c7e19e7c1012e)

Please check out our [Discord channel #hack-evening-2024-4](https://discord.com/channels/704612189532586014/1315802468868817007) for further help and assistance.

## Additional Resources

- [Github project](https://github.com/rust-meetup-munich/hack-evening-2024-4)
- [event description on meetup.com](https://www.meetup.com/rust-munich/events/304827279/)
- [Rust standard documentation](https://doc.rust-lang.org/std/)
- [original One Billion Challenge blog post](https://www.morling.dev/blog/one-billion-row-challenge/)
- [One Billion Row Challenge website](https://1brc.dev/)
- [flamegraph installation instructions](https://github.com/flamegraph-rs/flamegraph)
