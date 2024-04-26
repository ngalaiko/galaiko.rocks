pub struct Image {
    img: image::DynamicImage,
}

impl TryFrom<&[u8]> for Image {
    type Error = ImageError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let img = image::load_from_memory(value).map_err(ImageError)?;
        Ok(Self { img })
    }
}

impl Image {
    #[allow(clippy::missing_errors_doc)]
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ImageError> {
        let img = image::open(path).map_err(ImageError)?;
        Ok(Self { img })
    }

    #[must_use]
    pub fn resize(&self, width: Option<u32>, height: Option<u32>) -> Image {
        let img = self.img.resize(
            width.map_or_else(|| self.img.width(), |width| width * 2),
            height.map_or_else(|| self.img.height(), |height| height * 2),
            image::imageops::FilterType::Triangle,
        );

        Image { img }
    }

    #[must_use]
    pub fn webp(&self, quality: f32) -> Vec<u8> {
        let img = self.img.to_rgba8();
        let (width, height) = img.dimensions();
        webp::Encoder::new(&img, webp::PixelLayout::Rgba, width, height)
            .encode(quality)
            .to_vec()
    }
}

#[derive(Debug)]
pub struct ImageError(image::ImageError);

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ImageError {}
