pub(crate) fn try_buffer(pixels: usize, bytes_per_pixel: usize) -> Result<Vec<u8>, IoError> {
    let len = pixels.checked_mul(bytes_per_pixel).ok_or_else(|| IoError::Unsupported("image too large".into()))?;
    let mut v = Vec::new();
    v.try_reserve_exact(len).map_err(|_| IoError::Unsupported(format!("not enough memory for a {} MB image", len >> 20)))?;
    Ok(v)
}

fn map_bands(img: &Image, layout: ChannelLayout, sample: CSample, f: impl Fn(Vec<f32>) -> Vec<f32>) -> Result<Image, IoError> {
    let (w, h) = img.dimensions();
    let row = img.data().len() / (h.max(1) as usize);
    let band = row.max(1) * (BAND_BYTES / row.max(1)).max(1);
    let mut data = try_buffer(img.pixel_count(), layout.channels() * sample.bytes())?;
    for rows in img.data().chunks(band) {
        let n = (rows.len() / row.max(1)) as u32;
        let vals = f(Image::from_raw(w, n, img.layout(), img.sample_type(), rows.to_vec())?.to_normalized());
        data.extend_from_slice(Image::from_normalized(w, n, layout, sample, &vals)?.data());
    }
    Ok(Image::from_raw(w, h, layout, sample, data)?.with_icc(img.icc.clone()).with_meta(img.meta.clone()))
}

fn convert_rgb(img: &Image, dst: &photocraft_cms::Profile, intent: photocraft_cms::Intent, bpc: bool, sample: CSample) -> Result<Option<Image>, IoError> {
    use photocraft_cms::{Builtin, ColorSpace, Profile};
    if !img.layout().is_rgb() {
        return Ok(None);
    }
    let src =
        img.icc.as_ref().and_then(|b| Profile::parse(b).ok()).filter(|p| p.color_space == ColorSpace::Rgb).unwrap_or_else(|| Builtin::Srgb.profile().clone());
    convert_from(img, &src, dst, intent, bpc, sample)
}

/// Gray pixels converted from their profile (sGray when untagged) to `dst`, unclamped, untagged
/// and stored as `sample`. `None` when the image isn't gray or already holds `dst`'s colours.
fn convert_gray(img: &Image, dst: &photocraft_cms::Profile, sample: CSample) -> Result<Option<Image>, IoError> {
    use photocraft_cms::{Builtin, ColorSpace, Intent, Profile};
    if !matches!(img.layout(), ChannelLayout::Gray | ChannelLayout::GrayA) {
        return Ok(None);
    }
    let src =
        img.icc.as_ref().and_then(|b| Profile::parse(b).ok()).filter(|p| p.color_space == ColorSpace::Gray).unwrap_or_else(|| Builtin::SGray.profile().clone());
    convert_from(img, &src, dst, Intent::RelativeColorimetric, false, sample)
}

/// `img`'s pixels converted from `src` to `dst` (see [`convert_rgb`]).
fn convert_from(
    img: &Image,
    src: &photocraft_cms::Profile,
    dst: &photocraft_cms::Profile,
    intent: photocraft_cms::Intent,
    bpc: bool,
    sample: CSample,
) -> Result<Option<Image>, IoError> {
    if src.same_colors(dst) {
        return Ok(None);
    }
    let t = photocraft_cms::Transform::new(src, dst, intent, bpc).map_err(|e| IoError::Unsupported(e.to_string()))?;
    let stride = img.layout().channels();
    let out = map_bands(img, img.layout(), sample, |mut vals| {
        t.apply(&mut vals, stride);
        vals
    })?;
    Ok(Some(out.with_icc(None)))
}
