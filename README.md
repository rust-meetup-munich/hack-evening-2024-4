# Billion Row Challenge <!-- omit in toc -->

> This challenge is inspired by the initial [Java version](https://1brc.dev/), including the rules,
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
  - [Helpful Commands](#helpful-commands)
  - [Project Structure](#project-structure)
  - [Tips \& Tricks](#tips--tricks)
  - [Advanced Techniques](#advanced-techniques)
- [Further Resources](#further-resources)

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
{Abha=5.0/18.0/27.4, Abidjan=15.7/26.0/34.1, Abéché=12.1/29.4/35.6, Accra=14.7/26.4/33.1, Addis Ababa=2.1/16.0/24.3, Adelaide=4.1/17.3/29.7, ...}
```

Oh, and this input.txt is different for each submission since it's generated on-demand. So no
hard-coding the results! 😉

### Rules and Limitations

[original source](https://1brc.dev/#rules-and-limits)

1. You may use external libraries (like optimized data structures, parsers, etc.) with the exception
   of libraries that can solve the full challenge by themselves (e.g. polars, embedded DuckDB, etc.)
2. Implementations must implemented in main.rs/utils.rs only. Try to keep it relatively short; don't
   copy-paste a forbidden library into your solution as a cheat.
3. The computation must happen at application runtime; you cannot process the measurements file at
   build time.
4. Input value ranges are as follows:
   - Station name: non null UTF-8 string of min length 1 character and max length 100 bytes (i.e.
     this could be 100 one-byte characters, or 50 two-byte characters, etc.)
   - Temperature value: non null double between -99.9 (inclusive) and 99.9 (inclusive), always with
     one fractional digit
5. There is a maximum of 10,000 unique station names.
6. Implementations must not rely on specifics of a given data set. Any valid station name as per the
   constraints above and any data distribution (number of measurements per station) must be
   supported.

## Project Particularities

### Helpful Commands

| Task                     | Command                                                                      |
| ------------------------ | ---------------------------------------------------------------------------- |
| generate 1B rows of data | `cargo run --bin data-generator --release -- --rows 1000000000`              |
| run a debug build        | `cargo run           -- --input-file weather_1B.csv`                         |
| run a release build      | `cargo run --release -- --input-file weather_1B.csv`                         |
| run tests                | `cargo test`                                                                 |
| run flamegraph           | `sudo cargo flamegraph --root --bin solution -- --input-file weather_1B.csv` |

### Project Structure

TODO

### Tips & Tricks

- Go for a functionally correct, simple solution first and then iteratively improve parts.
- Keep your intermediate solutions by committing them, both for discussions and comparisons.
- Use a profiler like [flamegraph](https://github.com/flamegraph-rs/flamegraph) to find out the
  actual bottlenecks in your program.
- When developing, use debug builds and smaller datasets (e.g. 10M rows) to avoid long computation
  times.

### Advanced Techniques

This section lists a few advanced techniques you may want to consider:

- I/O techniques
  - buffered I/O
  - memory mapped files
  - asynchronous I/O with io_uring
- use preallocations and avoid additional ones
- implement specialized float parsing
- custom hash functions
- concurrency
  - reading the file with multiple threads in parallel
  - concurrent hashmaps like dash
  - SIMD instructions for parsing

## Further Resources

TODO: Add links to Rust documentation, etc.

- [original One Billion Challenge blog post](https://www.morling.dev/blog/one-billion-row-challenge/)
- [One Billion Row Challenge website](https://1brc.dev/)
- [flamegraph installation instructions](https://github.com/flamegraph-rs/flamegraph)
