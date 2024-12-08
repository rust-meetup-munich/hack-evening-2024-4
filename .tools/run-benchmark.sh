#!/usr/bin/env bash

set -e # exit on first error
set -u # exit on using unset variable

# the first argument is the name of a folder that contains the solution to run
SOLUTION_FOLDER=$1
NAME=$(basename $SOLUTION_FOLDER)

# check that the provided folder exists and is a cargo project
if [ ! -d $SOLUTION_FOLDER ]; then
    echo "The provided folder does not exist: $SOLUTION_FOLDER"
    exit 1
fi

cd $SOLUTION_FOLDER

# first build the solution
cargo build --release
if [ $? -ne 0 ]; then
    echo "Failed to build the solution"
    exit 1
fi

hyperfine --warmup 0 --runs 5 \
    --export-json ../guide/src/20_leaderboard/results/$NAME.json \
    --show-output \
    'cargo run --release -- ../samples/weather_1B.csv'