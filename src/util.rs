#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
    
}

#[inline]
pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

#[inline]
pub fn random_f64_interval(min: f64, max: f64) -> f64 {
    return min + (max - min) * random_f64();
}