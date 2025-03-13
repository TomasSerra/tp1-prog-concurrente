pub fn leibniz_pi(iterations: u128) -> f64 {
    let mut pi = 0.0;
    for k in 0..iterations {
        let term = 4.0 * (-1.0_f64).powi(k as i32) / (2 * k + 1) as f64;
        pi += term;
    }
    pi
}