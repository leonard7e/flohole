use clap::Parser;
use std::collections::{BTreeMap, HashMap};

mod argument_parser;
mod scales;

fn fingerhole_locations(
    fraction: f64,
    pos: f64,
    notes: &[f64],
    pitch: &BTreeMap<usize, f64>,
) -> Vec<f64> {
    notes
        .iter()
        .enumerate()
        .map(|(idx, &note)| {
            pitch
                .get(&idx)
                .map(|t| fraction.powf((note + t / 100.0) / pos))
                .unwrap_or(fraction.powf(note / pos))
        })
        .collect()
}

fn calculate_fingerhole_positions(
    labium: f64,
    length: f64,
    fraction: f64,
    position_at_fraction: f64,
    scale: &[f64],
    pitch: &BTreeMap<usize, f64>,
) -> Vec<f64> {
    let fingerholes = fingerhole_locations(fraction, position_at_fraction, scale, pitch);
    fingerholes
        .iter()
        .map(|&p| labium + (length - labium) * p)
        .collect()
}

fn flute_scale(args: &argument_parser::Args, scale_layout: HashMap<&str, Vec<f64>>) -> Vec<f64> {
    scale_layout
        .get(args.scale.as_str())
        .expect("Invalid scale")
        .iter()
        .cycle()
        .scan(0.0f64, |st, elem| {
            let ret = *st;
            *st += elem;
            Some(ret)
        })
        .skip(1)
        .take(args.nholes)
        .collect()
}

fn report_results_to_user(args: &argument_parser::Args, holes: &[f64]) {
    println!("Flute Design Parameters:");
    println!("  Length: {:.2} mm", args.length);
    println!("  Labium: {:.2} mm", args.labium);
    println!("  Scale: {}", args.scale);
    println!("  Number of Fingerholes: {}", args.nholes);
    println!("\nAcoustic Model Parameters:");
    println!("  Fraction: {:.4}", args.fraction);
    println!("  Position (Halftones): {:.2}", args.position);

    if !args.tune.pitch.is_empty() {
        println!("\nTuning Adjustments (cents):");
        for (hole_index, pitch_adjustment) in &args.tune.pitch {
            println!("  Hole {}: {:.2} cents", hole_index + 1, pitch_adjustment);
        }
    } else {
        println!("\nNo tuning adjustments applied.");
    }

    println!("\nFingerhole Drilling Positions (mm):");
    for (i, &hole_position) in holes.iter().enumerate() {
        println!("  Hole {}: {:.2} mm", i + 1, hole_position);
    }
}

fn main() {
    let args = argument_parser::Args::parse();

    let scale_layout = scales::scale_layout();

    let flute_scale: Vec<f64> = flute_scale(&args, scale_layout);

    let holes = calculate_fingerhole_positions(
        args.labium,
        args.length,
        args.fraction,
        args.position,
        &flute_scale,
        &args.tune.pitch,
    );

    report_results_to_user(&args, &holes);
}
