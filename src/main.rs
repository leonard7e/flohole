use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Berechne die Position der Bohrlöcher für eine Flöte."
)]
struct Args {
    /// Länge der Flöte
    length: f64,

    /// Die Position der Tonleitern in dem Segment (Fraction) der Flöte
    #[arg(short, long, default_value_t = 9.0)]
    position: f64,

    /// Anzahl der Bohrlöcher,
    #[arg(short, long, default_value_t = 6)]
    nholes: usize,

    // Fraction bezeichnet ein Segment der Flöte.
    #[arg(short, long, default_value_t = 0.5)]
    fraction: f64,

    /// Layout der Bohrlöcher (dur6, moll6, dur7, moll7, dur8, moll8, chromatic, shakuhachi)
    #[arg(short, long, default_value = "major")]
    scale: String,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    labium: f64,
}

fn fingerhole_locations(fraction: f64, pos: f64, notes: &[f64]) -> Vec<f64> {
    notes
        .iter()
        .map(|&note| fraction.powf(note / pos))
        .collect()
}

fn bohrlöcher(labium: f64, length: f64, fraction: f64, pos: f64, skala: &[f64]) -> Vec<f64> {
    let fingerholes = fingerhole_locations(fraction, pos, skala);
    fingerholes
        .iter()
        .map(|&p| labium + (length - labium) * p)
        .collect()
}

fn get_drill_layout() -> HashMap<&'static str, Vec<f64>> {
    let mut drill_layout: HashMap<&str, Vec<f64>> = HashMap::new();
    drill_layout.insert("major", vec![2.0, 4.0, 5.0, 7.0, 9.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 12.0, 14.0]);
    drill_layout.insert("h-minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("z-minor", vec![2.0, 3.0, 4.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor-pentatonic", vec![3.0, 5.0, 7.0, 10.0, 12.0]);
    drill_layout.insert("major-pentatonic", vec![2.0, 4.0, 7.0, 9.0, 12.0]);
    drill_layout.insert("uniform", vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    drill_layout
}

/*
fn lerp(a1: f64, a2: f64, t: f64) -> f64 {
    (1.0 - t) * a1 + t * a2
}
*/

fn main() {
    let args = Args::parse();

    let drill_layout = get_drill_layout();
    let skala: Vec<f64> = drill_layout
        .get(args.scale.as_str())
        .expect("Invalid scale")
        .iter()
        .take(args.nholes)
        .cloned()
        .collect();
    let loecher = bohrlöcher(
        args.labium,
        args.length,
        args.fraction,
        args.position,
        &skala,
    );

    println!("{:?}", loecher);
}
