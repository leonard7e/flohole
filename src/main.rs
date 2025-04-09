use clap::{Parser, parser::ValueSource};
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

    /// Place midpoint of flute on this halftone
    #[arg(short, long, default_value_t = 9.0)]
    mid: f64,

    /// Layout der Bohrlöcher (dur6, moll6, dur7, moll7, dur8, moll8, chromatic, shakuhachi)
    #[arg(short, long, default_value = "minor")]
    scale: String,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 8.0)]
    labium: f64,
}

fn intervall(mid: f64, notes: &[f64]) -> Vec<f64> {
    notes.iter().map(|&note| 2.0f64.powf(-note / mid)).collect()
}

fn bohrlöcher(labium: f64, length: f64, mid: f64, skala: &[f64]) -> Vec<f64> {
    let intervall_werte = intervall(mid, skala);
    intervall_werte
        .iter()
        .map(|&iv| labium + (length - labium) * iv)
        .collect()
}

fn get_drill_layout() -> HashMap<&'static str, Vec<f64>> {
    let mut drill_layout: HashMap<&str, Vec<f64>> = HashMap::new();
    drill_layout.insert("mayor", vec![2.0, 4.0, 5.0, 7.0, 9.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 12.0, 14.0]);
    drill_layout.insert("h-minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("z-minor", vec![2.0, 3.0, 4.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor-pentatonic", vec![3.0, 5.0, 7.0, 10.0, 12.0]);
    drill_layout.insert("major-pentatonic", vec![2.0, 4.0, 7.0, 9.0, 12.0]);
    drill_layout.insert("uniform", vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    drill_layout
}

fn lerp(a1: f64, a2: f64, t:) -> f64 {
     (1.0-t)*a1 + t*a2
}

fn main() {
    let args = Args::parse();

    let drill_layout = get_drill_layout();
    let skala: Vec<f64> = drill_layout
        .get(args.scale.as_str())
        .expect("Invalid scale")
        .iter()
        .take(6)
        .cloned()
        .collect();
    let loecher = bohrlöcher(args.labium, args.length, args.mid, &skala);

    println!("{:?}", loecher);
}
