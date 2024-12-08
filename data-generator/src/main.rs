//! CLI for generating weather measurements based on logic translated from original
//! [One Billion Row challenge](https://github.com/gunnarmorling/1brc/blob/main/src/main/java/dev/morling/onebrc/CreateMeasurements.java)

use std::io::{BufWriter, Write};
use std::{fs::File, path::PathBuf};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};

use crate::data::STATIONS;

mod data;

/// Returns the default output folder for generated data.
///
/// Considers the `data` folder as the default if run via cargo, the standalone binary uses the
/// current directory.
fn get_default_folder() -> PathBuf {
    let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return PathBuf::from(".");
    };

    PathBuf::from(manifest_dir)
        .parent()
        .expect("Unknown cargo project structure")
        .join("samples")
}

/// CLI generating input data (weather measurements) for the Billion Row Challenge.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of rows to generate.
    #[arg(short, long)]
    rows: usize,

    /// Path to output folder.
    #[arg(long, default_value_os_t = get_default_folder())]
    output_folder: PathBuf,

    /// Path to output file.
    #[arg(short, long)]
    output_file: Option<String>,

    /// Optional seed for initializing the random number generator.
    #[arg(short, long)]
    seed: Option<u64>,

    /// Whether to skip interactive output (for use in CI).
    #[arg(short, long)]
    non_interactive: bool,
}

fn get_human_readable_number(mut number: usize) -> String {
    let mut suffix = "";
    if number >= 1_000_000_000 {
        number /= 1_000_000_000;
        suffix = "B";
    } else if number >= 1_000_000 {
        number /= 1_000_000;
        suffix = "M";
    } else if number >= 1_000 {
        number /= 1_000;
        suffix = "K";
    }

    format!("{}{}", number, suffix)
}

fn main() {
    let args = Cli::parse();
    let human_rows = get_human_readable_number(args.rows);

    let output_file = {
        let file = args
            .output_file
            .unwrap_or_else(|| format!("weather_{}.csv", &human_rows));
        let output_path = args.output_folder.join(file);
        println!("Writing {} rows to {}", human_rows, output_path.display());

        File::create(&output_path).expect("Failed to create output file")
    };

    let seed = args.seed.unwrap_or_else(|| rand::thread_rng().next_u64());
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut pb = if args.non_interactive {
        None
    } else {
        let pb = ProgressBar::new(args.rows as u64);

        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} (remaining: {eta})",
            )
            .unwrap(),
        );

        Some(pb)
    };

    let mut output_file = BufWriter::new(output_file);
    for row in 0..args.rows {
        let station = STATIONS.choose(&mut rng).unwrap();
        let temperature = station.generate_measurement(&mut rng);
        writeln!(output_file, "{};{:.1}", station.name, temperature).expect("Failed to write row");

        if row % 10_000 == 0 {
            if let Some(pb) = &mut pb {
                pb.inc(10_000);
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Done!");
    }

    println!("Finished writing {} rows (seed={})", human_rows, seed);
}
