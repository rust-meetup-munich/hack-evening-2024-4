# Learning Opportunities

## CLI

There are a few rust libraries available to create a CLI for your solution. This is a great opportunity to learn how to use them. Here we want to suggest two libraries, `clap` and `argh`.

### Clap

You could use [clap](https://docs.rs/clap/latest/clap/) - A full-featured, fast Command Line Argument Parser for Rust, to create a CLI for your solution.

:Note: clap has a sort of a declarative syntax, which works with rust proc macros. It's very well documented in form of examples, and reduces the boilerplate code you have to write for parsing command line arguments. This is very useful to explore if you are new to clap. Besides you can still use the procedural API if you prefer.

### Alternative Argh

There is different library called [Argh](https://crates.io/crates/argh) that comes with a declarative syntax for defining command line arguments. It's a bit less feature-rich than clap, yet it's powerful and easy to use.

## Ideas for your CLI

Here are some optional ideas for CLI options you could implement:

- support a `--help` flag to print a help message
- support multiple input files at once (e.g. `file1.csv file2.csv file3.csv`)
- support different output formats, e.g. JSON, CSV, etc. (e.g. `--output-format json`)
- support different output destinations, e.g. file (e.g. `--output-file output.csv`)
- support different output verbosity levels (e.g. `--verbose`, `--debug`, etc.)
- support different error handling strategies (e.g. `--ignore-errors`, `--fail-fast`, etc.)
