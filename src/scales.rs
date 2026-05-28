use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Scale {
    Major,
    Minor,
    Hminor,
    Zminor,
    Pentatonic,
    #[value(name = "whole-tone")]
    WholeTone,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no possible value")
            .get_name()
            .fmt(f)
    }
}

impl Scale {
    pub fn layout(&self) -> Vec<f64> {
        let tetrachord = vec![2.0, 2.0, 1.0];
        let pentatonic = vec![3.0, 2.0, 2.0, 3.0, 2.0];

        // The major scale
        let major = [tetrachord.clone(), vec![2.0], tetrachord.clone()].concat();
        let mut minor = major.clone();
        minor.rotate_right(2);

        let mut h_minor = minor.clone();
        h_minor[6] += 1.0;

        let mut z_minor = h_minor.clone();
        z_minor[2] -= 1.0;

        match self {
            Scale::Major => major,
            Scale::Minor => minor,
            Scale::Hminor => h_minor,
            Scale::Zminor => z_minor,
            Scale::Pentatonic => pentatonic,
            Scale::WholeTone => vec![2.0],
        }
    }
}
