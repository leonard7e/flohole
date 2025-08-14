use std::collections::HashMap;

pub fn scale_layout() -> HashMap<&'static str, Vec<f64>> {
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

    // Assembling the scales
    let mut drill_layout: HashMap<&str, Vec<f64>> = HashMap::new();

    drill_layout.insert("major", major);
    drill_layout.insert("minor", minor);
    drill_layout.insert("hminor", h_minor);
    drill_layout.insert("zminor", z_minor);
    drill_layout.insert("pentatonic", pentatonic);

    drill_layout
}
