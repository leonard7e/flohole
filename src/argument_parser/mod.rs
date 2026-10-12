use crate::scales::Scale;
use clap::Parser;

pub mod drop;
pub mod tune;

use drop::Drop;
use tune::Tune;

// Adopt version from Cargo.toml
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Implement the argument parser
#[derive(Parser, Debug)]
#[command(
    author = "Leonard Siebeneicher",
    version = APP_VERSION,
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

    /// Key transposition in Semitones
    #[arg(short, long, default_value_t = 0.0)]
    pub key: f64,

    /// Layout of the holes
    #[arg(short, long, value_enum, default_value_t = Scale::Major)]
    pub scale: Scale,

    #[arg(short, long, default_value = "")]
    pub tune: Tune,

    #[arg(short, long, default_value = "")]
    pub drop: Drop,

    /// Depth of the labium
    #[arg(short, long, default_value_t = 6.0)]
    pub labium: f64,
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        Some(self.length)
            .filter(|&l| l > 0.0)
            .ok_or_else(|| "Length must be greater than zero.".to_string())?;

        Some(self.labium)
            .filter(|&l| l >= 0.0)
            .ok_or_else(|| "Labium depth must be greater or equal zero.".to_string())?;

        Some(())
            .filter(|_| self.length > self.labium)
            .ok_or_else(|| {
                format!(
                    "Length ({:.2} mm) must be greater than labium depth ({:.2} mm).",
                    self.length, self.labium
                )
            })?;

        Some(self.fraction)
            .filter(|&f| f > 0.0 && f < 1.0)
            .ok_or_else(|| "Fraction must be strictly between 0.0 and 1.0.".to_string())?;

        Some(self.position)
            .filter(|&p| p > 0.0)
            .ok_or_else(|| "Position must be greater than zero.".to_string())?;

        Some(self.nholes)
            .filter(|&n| n > 0)
            .ok_or_else(|| "Number of holes must be at least 1.".to_string())?;

        Some(self.key).filter(|&k| k >= 0.0).ok_or_else(|| {
            "Key transposition must be greater than or equal to zero.".to_string()
        })?;

        // Validate drop indices
        let scale_pattern = self.scale.layout();
        let scale_len = scale_pattern.len();
        if let Some(dropped) = self
            .drop
            .drop_from_scale
            .iter()
            .copied()
            .find(|&d| d >= scale_len)
        {
            return Err(format!(
                "Drop index {} is out of bounds for scale '{}' (valid indices: 0..{})",
                dropped, self.scale, scale_len
            ));
        }

        Some(self.drop.drop_from_scale.len())
            .filter(|&len| len < scale_len)
            .ok_or_else(|| {
                format!(
                    "Cannot drop all elements of scale '{}'. At least one interval must remain.",
                    self.scale
                )
            })?;

        // Validate tuning hole indices (1-based index)
        self.tune.pitch
            .keys()
            .copied()
            .find(|&hole| hole == 0 || hole > self.nholes)
            .map(|hole| -> Result<(), String> {
                let msg = Some(hole)
                    .filter(|&h| h == 0)
                    .map(|_| "Tuning adjustment specified for hole 0, but holes are 1-indexed.".to_string())
                    .unwrap_or_else(|| format!(
                        "Tuning adjustment specified for hole {}, but the flute only has {} holes.",
                        hole, self.nholes
                    ));
                Err(msg)
            })
            .transpose()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn base_valid_args() -> Args {
        Args {
            length: 300.0,
            position: 9.0,
            nholes: 6,
            fraction: 0.5,
            scale: Scale::Major,
            tune: Tune::default(),
            drop: Drop::default(),
            labium: 6.0,
            key: 0.0,
        }
    }

    #[test]
    fn test_valid_args() {
        let args = base_valid_args();
        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_invalid_length() {
        let mut args = base_valid_args();
        args.length = 0.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Length must be greater than zero."
        );

        args.length = -10.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Length must be greater than zero."
        );
    }

    #[test]
    fn test_invalid_labium() {
        let mut args = base_valid_args();
        args.labium = -1.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Labium depth must be greater or equal zero."
        );
    }

    #[test]
    fn test_length_less_than_labium() {
        let mut args = base_valid_args();
        args.length = 5.0;
        args.labium = 6.0;
        assert!(
            args.validate()
                .unwrap_err()
                .contains("must be greater than labium depth")
        );
    }

    #[test]
    fn test_invalid_fraction() {
        let mut args = base_valid_args();
        args.fraction = 0.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Fraction must be strictly between 0.0 and 1.0."
        );

        args.fraction = 1.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Fraction must be strictly between 0.0 and 1.0."
        );

        args.fraction = 1.5;
        assert_eq!(
            args.validate().unwrap_err(),
            "Fraction must be strictly between 0.0 and 1.0."
        );
    }

    #[test]
    fn test_invalid_position() {
        let mut args = base_valid_args();
        args.position = 0.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Position must be greater than zero."
        );

        args.position = -2.0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Position must be greater than zero."
        );
    }

    #[test]
    fn test_invalid_nholes() {
        let mut args = base_valid_args();
        args.nholes = 0;
        assert_eq!(
            args.validate().unwrap_err(),
            "Number of holes must be at least 1."
        );
    }

    #[test]
    fn test_invalid_scale() {
        // clap handles invalid values automatically;
        // manual validation of Scale is no longer possible because it's an Enum.
    }

    #[test]
    fn test_invalid_drop_out_of_bounds() {
        let mut args = base_valid_args();
        // major scale layout has length 7 (intervals). So index 7 is out of bounds (valid indices: 0..7, which are 0 to 6)
        let mut drop_set = BTreeSet::new();
        drop_set.insert(7);
        args.drop = Drop {
            drop_from_scale: drop_set,
        };
        assert!(
            args.validate()
                .unwrap_err()
                .contains("Drop index 7 is out of bounds")
        );
    }

    #[test]
    fn test_invalid_drop_all() {
        let mut args = base_valid_args();
        // drop all indices for major scale (0..7 indices, total 7)
        let drop_set: BTreeSet<_> = (0..7).collect();
        args.drop = Drop {
            drop_from_scale: drop_set,
        };
        assert!(
            args.validate()
                .unwrap_err()
                .contains("Cannot drop all elements of scale")
        );
    }

    #[test]
    fn test_invalid_tune_zero() {
        let mut args = base_valid_args();
        let mut pitch_map = std::collections::BTreeMap::new();
        pitch_map.insert(0, 10.0);
        args.tune = Tune { pitch: pitch_map };
        assert!(args.validate().unwrap_err().contains("holes are 1-indexed"));
    }

    #[test]
    fn test_invalid_tune_out_of_bounds() {
        let mut args = base_valid_args();
        let mut pitch_map = std::collections::BTreeMap::new();
        pitch_map.insert(7, 10.0); // 7 is greater than 6 nholes
        args.tune = Tune { pitch: pitch_map };
        assert!(
            args.validate()
                .unwrap_err()
                .contains("the flute only has 6 holes")
        );
    }
}
