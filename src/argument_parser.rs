use clap::Parser;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct Tune {
    pub pitch: Vec<f64>,
}

impl Default for Tune {
    fn default() -> Self {
        Tune {
            pitch: vec![0.0; 20],
        }
    }
}

impl Display for Tune {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tune: {:?}", self.pitch)
    }
}

impl FromStr for Tune {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tune_map: BTreeMap<usize, f64> = BTreeMap::new();
        for (idx, entry) in s.split(":").map(|e| {
            let entry = e.split(',').collect::<Vec<&str>>();
            println!("entry: {:?}", entry);
            (
                entry[0].parse::<usize>().unwrap(),
                entry[1].parse::<f64>().unwrap(),
            )
        }) {
            if let Some(value) = tune_map.get(&idx) {
                tune_map.insert(idx, entry + value);
            } else {
                tune_map.insert(idx, entry);
            }
        }

        Ok(Tune {
            pitch: (0..20)
                .map(|idx| tune_map.get(&idx).unwrap_or(&0.0))
                .cloned()
                .collect(),
        })
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

    #[arg(short, long, default_value = "0,0")]
    pub tune: Tune,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}
