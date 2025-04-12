use clap::Parser;
use std::fmt::Display;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct Tune {
    pub tune: Vec<(usize, f64)>,
}

impl Default for Tune {
    fn default() -> Self {
        Tune {
            tune: vec![(0, 0.0), (1, 0.0), (2, 0.0), (3, 0.0), (4, 0.0), (5, 0.0)],
        }
    }
}

impl Display for Tune {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tune: {:?}", self.tune)
    }
}

impl FromStr for Tune {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = s
            .split(":")
            .map(|e| {
                let entry = e.split(',').collect::<Vec<&str>>();
                (
                    entry[0].parse::<usize>().unwrap(),
                    entry[1].parse::<f64>().unwrap(),
                )
            })
            .collect::<Vec<(usize, f64)>>();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(Tune { tune: entries })
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = "0.1.0",
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

    #[arg(short, long, default_value_t = Tune::default())]
    pub tune: Tune,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}
