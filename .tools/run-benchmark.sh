#!/usr/bin/env bash

set -e # exit on first error
set -u # exit on using unset variable

# the first argument is the name of a folder that contains the solution to run
SOLUTION_FOLDER=$1
NAME=$(basename $SOLUTION_FOLDER)

# IMPORTANT: all folders below are relative to the solution folder as root folder!

BENCHMARK_DATASET=../samples/weather_1B.csv
RESULTS_FOLDER=../guide/src/20_leaderboard/results

# check that the provided folder exists and is a cargo project
if [ ! -d $SOLUTION_FOLDER ]; then
    echo "The provided folder does not exist: $SOLUTION_FOLDER"
    exit 1
fi
cd $SOLUTION_FOLDER

# check that the results folder exists, if not create it
if [ ! -d $RESULTS_FOLDER ]; then
    echo "[warn] The results folder does not exist: $RESULTS_FOLDER, creating it"
    mkdir -p $RESULTS_FOLDER
fi

# check that the benchmark dataset of 1B rows exists
if [ ! -f $BENCHMARK_DATASET ]; then
    echo "The benchmark dataset of 1B rows does not exist"
    exit 1
fi

# first build the solution
cargo build --release
if [ $? -ne 0 ]; then
    echo "Failed to build the solution"
    exit 1
fi

hyperfine --warmup 0 --runs 5 \
    --export-json $RESULTS_FOLDER/$NAME.json \
    --show-output \
    "cargo run --release -- $BENCHMARK_DATASET"