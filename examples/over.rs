use anyhow::Result;
use openimageio::{FromFileOptions, ImageBuf, ImageCache, Utf8Path};

fn main() -> Result<()> {
    // Create a shared cache that will persist after this instance gets dropped.
    // Cloning an `ImageCache` is cheap and the clone will refer to the same
    // underlying data.
    let image_cache = ImageCache::shared(false);

    // Load fg image. This is 1024×1024
    let mut image_buf_a = ImageBuf::from_file_with(
        Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
        &FromFileOptions {
            image_cache: Some(image_cache.clone()),
            ..Default::default()
        },
    )?;

    // Load bg image. This is 2048×1024.
    let image_buf_b = ImageBuf::from_file_with(
        Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"),
        &FromFileOptions {
            image_cache: Some(image_cache),
            ..Default::default()
        },
    )?;

    // Compose fg over bg, replacing the data window of fg  with the result.
    // I.e. the result will be cropped at fg's original dimensions of 1024×1024.
    image_buf_a.over(&image_buf_b);

    // Write the result. The file format is inferred from the extension.
    image_buf_a.write(Utf8Path::new("over.exr"))?;

    Ok(())
}
