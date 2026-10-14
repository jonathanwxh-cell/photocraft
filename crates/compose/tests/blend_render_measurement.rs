//! Investigation-only CPU fallback cost; excludes GPU, uploads and UI event latency.
use photocraft_color::{BlendMode, Color, ColorMode, PixelFormat, SampleType};
use photocraft_compose::render;
use photocraft_doc::{Document, Layer, LayerContent};
use photocraft_geom::{Rect, Size};
use photocraft_raster::Surface;
use std::{hint::black_box, time::Instant};

#[test]
#[ignore]
fn blend_render_region_measurement() {
    let mut doc = Document::with_background("synthetic", Size::new(6000, 4000), ColorMode::Rgb, SampleType::U8, Color::WHITE);
    for seed in [3u8, 7] {
        let mut surface = Surface::new(PixelFormat::RGBA8);
        let row: Vec<u8> = (0..6000u32).flat_map(|x| [(x as u8).wrapping_mul(seed), (x / 3) as u8, seed * 20, 200]).collect();
        for y in 0..4000 {
            surface.write_interleaved(Rect::new(0, y, 6000, y + 1), &row);
        }
        doc.layers.push(Layer::new("pattern", LayerContent::Raster(surface)));
    }
    let visible = Rect::new(1024, 1024, 2304, 1744);
    for mode in [BlendMode::Multiply, BlendMode::Screen] {
        doc.layers.last_mut().unwrap().blend = mode;
        // Untimed warmup and full pixel equivalence for this synthetic fixture.
        let full = render(&doc, doc.bounds());
        let partial = render(&doc, visible);
        for y in visible.y0..visible.y1 {
            for x in visible.x0..visible.x1 {
                assert_eq!(full.get(x, y), partial.get(x, y));
            }
        }
        drop((full, partial));
        for sample in 0..5 {
            // Reverse the order on alternating samples to reduce order bias.
            let regions = if sample % 2 == 0 { [doc.bounds(), visible] } else { [visible, doc.bounds()] };
            for region in regions {
                let start = Instant::now();
                let result = render(&doc, region);
                let us = start.elapsed().as_micros();
                black_box(&result);
                println!("BLEND_RENDER mode={mode:?} sample={sample} width={} height={} elapsed_us={us}", region.width(), region.height());
            }
        }
    }
}
