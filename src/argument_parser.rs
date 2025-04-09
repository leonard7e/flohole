use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = "0.1.0",
    about = "Berechne die Position der Bohrlöcher für eine Flöte.",
    long_about = "Berechne die Position der Bohrlöcher für eine Flöte."
)]

pub struct Args {
    /// Länge der Flöte
    pub length: f64,

    /// Die Position der Tonleitern in dem Segment (Fraction) der Flöte
    #[arg(short, long, default_value_t = 9.0)]
    pub position: f64,

    /// Anzahl der Bohrlöcher,
    #[arg(short, long, default_value_t = 6)]
    pub nholes: usize,

    /// Fraction bezeichnet ein Segment der Flöte.
    #[arg(short, long, default_value_t = 0.5)]
    pub fraction: f64,

    /// Layout der Bohrlöcher (dur6, moll6, dur7, moll7, dur8, moll8, chromatic, shakuhachi)
    #[arg(short, long, default_value = "major")]
    pub scale: String,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}
