
use std::collections::HashMap;

pub fn scale_layout() -> HashMap<&'static str, Vec<f64>> {
    let mut drill_layout: HashMap<&str, Vec<f64>> = HashMap::new();
    drill_layout.insert("major", vec![2.0, 4.0, 5.0, 7.0, 9.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 10.0, 12.0, 14.0]);
    drill_layout.insert("h-minor", vec![2.0, 3.0, 5.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("z-minor", vec![2.0, 3.0, 4.0, 7.0, 8.0, 11.0, 12.0, 14.0]);
    drill_layout.insert("minor-pentatonic", vec![3.0, 5.0, 7.0, 10.0, 12.0]);
    drill_layout.insert("major-pentatonic", vec![2.0, 4.0, 7.0, 9.0, 12.0]);
    drill_layout.insert("uniform", vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    drill_layout
}
