
#[cfg(test)]
mod noise_cloud_evidence {
    use super::*;
    #[test]
    #[ignore]
    fn noise_cloud_evidence() {
        let label = std::env::var("NOISE_LABEL").unwrap();
        let mut small = Map::new(96, 64, 0.0);
        for y in 8..48 {
            for x in 8..48 {
                small.v[y * 96 + x] = ((x as f32 - y as f32 * 0.7) * 0.3).clamp(0.0, 1.0);
            }
        }
        blur(&mut small, 5.0);
        noise(&mut small, 0.26, 0, 0);
        let mut ppm = b"P6\n96 64\n255\n".to_vec();
        for v in &small.v {
            let gray = ((1.0 - v.clamp(0.0, 1.0)) * 255.0).round() as u8;
            ppm.extend_from_slice(&[gray; 3]);
        }
        std::fs::write(format!("../../evidence/{label}.ppm"), ppm).unwrap();
        let mut shape = Map::new(6000, 4000, 0.0);
        for y in 400..3600 {
            for x in 400..5600 {
                shape.v[y * 6000 + x] = ((x as f32 - y as f32 * 0.7) * 0.3).clamp(0.0, 1.0);
            }
        }
        for width in [5.0, 32.0] {
            let mut times = Vec::new();
            for _ in 0..3 {
                let mut m = shape.clone();
                let start = std::time::Instant::now();
                blur(&mut m, width);
                noise(&mut m, 0.26, 0, 0);
                times.push(start.elapsed().as_secs_f64() * 1000.0);
                std::hint::black_box(m);
            }
            println!("{label}: 6000x4000 blur + noise, width={width}, milliseconds={times:?}");
        }
    }
}
