use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Drop {
    pub drop_from_scale: BTreeSet<usize>,
}

impl Default for Drop {
    fn default() -> Self {
        Drop {
            drop_from_scale: BTreeSet::new(),
        }
    }
}

impl Display for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Drop these: {:?}", self.drop_from_scale)
    }
}

impl FromStr for Drop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Drop::default());
        } else {
            s.split(",")
                .map(|s| s.parse::<usize>())
                .collect::<Result<BTreeSet<usize>, _>>()
                .map(|b| Drop { drop_from_scale: b })
                .map_err(|_| format!("Invalid drop value: {}", s))
        }
    }
}
