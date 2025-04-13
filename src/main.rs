use clap::Parser;
use std::collections::HashMap;

mod argument_parser;
mod scales;

fn fingerhole_locations(fraction: f64, pos: f64, notes: &[f64], pitch: &[f64]) -> Vec<f64> {
    notes
        .iter()
        .enumerate()
        .map(|(idx, &note)| fraction.powf((note + pitch[idx] / 100.0) / pos))
        .collect()
}

fn calculate_fingerhole_positions(
    labium: f64,
    length: f64,
    fraction: f64,
    position_at_fraction: f64,
    scale: &[f64],
    pitch: &[f64],
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
        .take(args.nholes)
        .cloned()
        .collect()
}

fn report_results_to_user(args: &argument_parser::Args, holes: &[f64]) {
    println!("Length: {}mm\tLabium: {:?}mm", args.length, args.labium);
    println!(
        "Fraction: {}\tPosition (Halftones): {}",
        args.fraction, args.position
    );
    println!(
        "Scale: {:?}\tAmount of fingerholes: {}\n",
        args.scale, args.nholes
    );
    println!("Tune fingerholes: {}\n", args.tune);
    println!("Fingerhole drilling Positions\n{:?}", holes);
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
