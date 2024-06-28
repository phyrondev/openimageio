use anyhow::Result;
use camino::Utf8Path;
use oiio::{ImageBuf, ImageCache, ImageSpec};

fn main() -> Result<()> {
    let mut image_buf_a = ImageBuf::from_file(
        &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
        None,
        None,
        None,
        None::<ImageSpec>,
    );

    let image_buf_b = ImageBuf::from_file(
        &Utf8Path::new("assets/wooden_lounge_2k.exr"),
        None,
        None,
        None,
        None::<ImageSpec>,
    );

    image_buf_a.over(&image_buf_b, None, Some(16));

    //image_buf_a.save("over.exr")?;

    Ok(())
}
