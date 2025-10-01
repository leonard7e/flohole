use clap::Parser;

pub mod drop;
pub mod tune;

use crate::version;
use drop::Drop;
use tune::Tune;

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = version::APP_VERSION,
    about = "Calculate the position of the holes for a flute.",
    long_about = "Calculate the position of the holes for a flute."
)]

pub struct Args {
    /// Length of the flute
    pub length: f64,

    /// The position of the tone scale in the segment (Fraction) of the flute
    #[arg(short, long, default_value_t = 9.0)]
    pub position: f64,

    /// Number of holes
    #[arg(short, long, default_value_t = 6)]
    pub nholes: usize,

    /// Fraction representing the segment of the flute
    #[arg(short, long, default_value_t = 0.5)]
    pub fraction: f64,

    /// Layout of the holes
    #[arg(short, long, default_value = "major")]
    pub scale: String,

    #[arg(short, long, default_value = "")]
    pub tune: Tune,

    #[arg(short, long, default_value = "")]
    pub drop: Drop,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}
