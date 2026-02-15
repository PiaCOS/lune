/// Evaluate Polynomal with Horner's Method.
///
/// * `t` - The value to evaluate the polynomial with.
/// * `coefs` - The polynomials terms sorted in increasing order.
pub fn polynomial_eval(t: f64, coefs: &[f64]) -> f64 {
    coefs.iter().rev().fold(0.0, |acc, &coef| acc * t + coef)
}

/// Converts an angle in radiants to degrees.
///
/// * `angle` - The angle in radiants.
pub fn map_to_deg(angle: f64) -> f64 {
    let m = angle % 360.0;
    if m < 0.0 { m + 360.0 } else { m }
}

/// Gives the initials of each words of the input.
///
/// * `text` - The input text
pub fn get_initials(text: &str) -> String {
    text.split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect()
}
