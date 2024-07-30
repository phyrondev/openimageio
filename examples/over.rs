use anyhow::Result;
use camino::Utf8Path as Path;
use oiio::{ImageBuf, ImageCache};
use openimageio as oiio;

fn main() -> Result<()> {
    // Create a shared cache that will persist after this
    // instance gets dropped.
    let image_cache = ImageCache::shared(false);

    // Load fg image. This is 1024×1024
    let mut image_buf_a =
        ImageBuf::from_file(Path::new("assets/j0.3toD__F16_RGBA.exr"));

    // Load bg image. This is 2048×1024.
    let image_buf_b =
        ImageBuf::from_file(Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"));

    // Compose fg over bg, replacing the data window of fg
    // with the result. I.e. the result will be cropped at
    // fg's original dimensions of 1024×1024.
    image_buf_a.over(&image_buf_b);

    // Write the result
    image_buf_a.write(Path::new("over.exr"), None, None)?;

    Ok(())
}
