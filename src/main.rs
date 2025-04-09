use clap::Parser;
use std::collections::HashMap;

mod argument_parser;
mod scales;

fn fingerhole_locations(fraction: f64, pos: f64, notes: &[f64]) -> Vec<f64> {
    notes
        .iter()
        .map(|&note| fraction.powf(note / pos))
        .collect()
}

fn drilling_holes(labium: f64, length: f64, fraction: f64, pos: f64, skala: &[f64]) -> Vec<f64> {
    let fingerholes = fingerhole_locations(fraction, pos, skala);
    fingerholes
        .iter()
        .map(|&p| labium + (length - labium) * p)
        .collect()
}

/*
fn lerp(a1: f64, a2: f64, t: f64) -> f64 {
    (1.0 - t) * a1 + t * a2
}
*/

fn flute_scale(args: &argument_parser::Args, scale_layout: HashMap<&str, Vec<f64>>) -> Vec<f64> {
    scale_layout
        .get(args.scale.as_str())
        .expect("Invalid scale")
        .iter()
        .take(args.nholes)
        .cloned()
        .collect()
}

fn main() {
    let args = argument_parser::Args::parse();

    let scale_layout = scales::scale_layout();

    let flute_scale: Vec<f64> = flute_scale(&args, scale_layout);

    let loecher = drilling_holes(
        args.labium,
        args.length,
        args.fraction,
        args.position,
        &flute_scale,
    );

    println!("{:?}", loecher);
}
