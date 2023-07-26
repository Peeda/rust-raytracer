use rand::Rng;
pub fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {return min};
    if x > max {return max};
    x
}
pub fn rand_float(min:f64, max:f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let a = rng.gen_range(-1000..0) as f64;
            let b = rng.gen_range(0..1000) as f64;
            let c = rand_float(a,b);
            assert!(c >= a && c < b);
        }
    }
}
