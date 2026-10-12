use photocraft_codecs::{Image, ChannelLayout, SampleType as CSample, Format, EncodeOptions};
use photocraft_cms::{Builtin, Intent};
use std::{hint::black_box, time::Instant};
const BAND_BYTES: usize = 32 << 20;
#[derive(Debug, thiserror::Error)]
enum IoError {
    #[error("unsupported: {0}")] Unsupported(String),
    #[error("{0}")] Codec(#[from] photocraft_codecs::CodecError),
}
include!("helpers.rs");
fn run(img: &Image, fixed: bool) -> usize {
    // Same before/after export branch. Input already comes from document_to_image.
    let mut converted = convert_rgb(img, Builtin::Srgb.profile(), Intent::Perceptual, true, img.sample_type()).unwrap();
    if fixed && converted.is_none() {
        converted = convert_gray(img, Builtin::SGray.profile(), img.sample_type()).unwrap();
    }
    let final_img = converted.as_ref().unwrap_or(img);
    let opts = EncodeOptions::default();
    black_box(photocraft_codecs::fidelity_warnings_with(final_img, Format::Tga, &opts));
    let bytes = photocraft_codecs::encode(final_img, Format::Tga, &opts).unwrap();
    black_box(bytes).len()
}
fn main() {
    let (w,h) = (6000,4000);
    let pixels: Vec<f32> = (0..w*h).map(|i| 0.05 + (i % 6000) as f32 / 7000.0).collect();
    let img = Image::from_normalized(w,h,ChannelLayout::Gray,CSample::F32,&pixels).unwrap()
        .with_icc(Some(photocraft_cms::builtin::linear_gray().to_bytes().to_vec()));
    drop(pixels);
    for fixed in [false,true] { black_box(run(&img,fixed)); }
    let mut samples = [Vec::new(),Vec::new()];
    for pair in 0..6 {
        let order = if pair%2==0 {[false,true]} else {[true,false]};
        for fixed in order {
            let now=Instant::now(); let len=run(&img,fixed); let ms=now.elapsed().as_secs_f64()*1000.;
            samples[usize::from(fixed)].push(ms);
            println!("pair{pair} fixed={fixed} ms={ms:.3} bytes={len}");
        }
    }
    for (i,values) in samples.iter_mut().enumerate() {
        values.sort_by(f64::total_cmp); println!("median fixed={} ms={:.3}", i==1, (values[2]+values[3])/2.);
    }
}
