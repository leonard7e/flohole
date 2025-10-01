use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use std::vec::Vec;

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
