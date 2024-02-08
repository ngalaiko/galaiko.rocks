use crate::{assets, path};

pub struct Image {
    pub path: std::path::PathBuf,
    img: image::DynamicImage,
}

impl TryFrom<&assets::Asset> for Image {
    type Error = ImageError;

    fn try_from(value: &assets::Asset) -> Result<Self, Self::Error> {
        let img = image::load_from_memory(&value.data).map_err(ImageError)?;
        Ok(Self {
            path: path::normalize(&value.path),
            img,
        })
    }
}

impl Image {
    #[must_use]
    pub fn resize(&self, width: Option<u32>, height: Option<u32>) -> Image {
        let img = self.img.resize(
            width.map_or_else(|| self.img.width(), |width| width * 2),
            height.map_or_else(|| self.img.height(), |height| height * 2),
            image::imageops::FilterType::Triangle,
        );

        let file_stem = self
            .path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .unwrap_or_default();

        let path = self.path.with_file_name(format!(
            "{file_stem}.{}x{}@2x.webp",
            width.unwrap_or(0),
            height.unwrap_or(0),
        ));

        Image { path, img }
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
