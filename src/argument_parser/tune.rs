use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
#[derive(Default)]
pub struct Tune {
    pub pitch: BTreeMap<usize, f64>,
}


impl Display for Tune {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tune: {:?}", self.pitch)
    }
}

impl FromStr for Tune {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Some(s)
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .map(|entry| {
                        let mut parts = entry.split('=');
                        let key_part = parts.next().ok_or_else(|| format!("Invalid tune entry: {}", entry))?;
                        let val_part = parts.next().ok_or_else(|| format!("Invalid tune entry: {}", entry))?;

                        Some(())
                            .filter(|_| parts.next().is_none())
                            .ok_or_else(|| format!("Invalid tune entry: {}", entry))?;

                        let hole_index_str = key_part.trim().replace("h", "");
                        let hole_index = hole_index_str
                            .parse::<usize>()
                            .map_err(|_| format!("Invalid hole index: {}", hole_index_str))?;

                        let adjustment_str = val_part.trim();
                        let adjustment = adjustment_str
                            .parse::<f64>()
                            .map_err(|_| format!("Invalid adjustment value: {}", adjustment_str))?;

                        Ok((hole_index, adjustment))
                    })
                    .collect::<Result<BTreeMap<usize, f64>, String>>()
            })
            .transpose()
            .map(|opt| Tune {
                pitch: opt.unwrap_or_default(),
            })
    }
}
