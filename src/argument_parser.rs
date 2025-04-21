use clap::Parser;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::vec::Vec;
// TODO: pitch lieber als BTreeMap Da brauche ich keine fixe Größe, wie etwa bei einem Vec.
#[derive(Clone, Debug)]
pub struct Tune {
    pub pitch: BTreeMap<usize, f64>,
}

impl Default for Tune {
    fn default() -> Self {
        Tune {
            pitch: BTreeMap::new(),
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
        if s.is_empty() {
            return Ok(Tune { pitch: tune_map });
        }

        for entry in s.split(',') {
            let parts: Vec<&str> = entry.split('=').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid tune entry: {}", entry));
            }

            let hole_index_str = parts[0].trim().replace("h", "");
            let hole_index = hole_index_str
                .parse::<usize>()
                .map_err(|_e| format!("Invalid hole index: {}", hole_index_str))?;

            let adjustment_str = parts[1].trim();
            let adjustment = adjustment_str
                .parse::<f64>()
                .map_err(|_e| format!("Invalid adjustment value: {}", adjustment_str))?;

            tune_map.insert(hole_index, adjustment);
        }

        Ok(Tune { pitch: tune_map })
    }
}

#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = "0.2.1",
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

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}
