use anyhow::Result;
use camino::Utf8Path;
use oiio::{ImageBuf, ImageCache, ImageSpec};

fn main() -> Result<()> {
    let image_cache = ImageCache::shared(false);

    let mut image_buf_a = ImageBuf::from_file(
        &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
        None,
        None,
        Some(&image_cache),
        None::<ImageSpec>,
    );

    let image_buf_b = ImageBuf::from_file(
        &Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"),
        None,
        None,
        Some(&image_cache),
        None::<ImageSpec>,
    );

    image_buf_a.over(&image_buf_b, None, None);

    image_buf_a.write(&Utf8Path::new("over.exr"), None, None)?;

    Ok(())
}
