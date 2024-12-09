# Submission

To submit your solution, please create a fork of this repository and submit a pull request to the `main` branch. Your pull request should include the following:

- A new directory in the `solutions` directory named after your GitHub username.
- This should contain a binary rust project with your solution.
- The binary should accept a single argument, that is the file path to the input file.

For example, if your GitHub username is `sassman`, your solution should be in `solutions/sassman`.
It should be callable like this:

```sh
cd solutions/sassman
cargo run --release -- ../../samples/weather_100.csv
```

The output should be written to stdout.

Please note that CI will run on your pull request to ensure that your solution compiles and runs correctly.

:NOTE: If you have any questions, feel free to ask in the `#hack-evening-2024-4` channel on the Rust Munich Discord server.

Good luck! 🦀
