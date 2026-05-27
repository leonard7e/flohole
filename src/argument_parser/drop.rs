use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
#[derive(Default)]
pub struct Drop {
    pub drop_from_scale: BTreeSet<usize>,
}


impl Display for Drop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Drop these: {:?}", self.drop_from_scale)
    }
}

impl FromStr for Drop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Some(s)
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .map(|item| item.parse::<usize>().map_err(|_| format!("Invalid drop value: {}", s)))
                    .collect::<Result<BTreeSet<usize>, String>>()
            })
            .transpose()
            .map(|opt| Drop {
                drop_from_scale: opt.unwrap_or_default(),
            })
    }
}
