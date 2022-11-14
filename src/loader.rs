use bevy::asset::{AssetLoader, LoadedAsset};
use bevy::prelude::*;
use bevy::render::texture::ImageType;

use crate::NineSlice;

pub struct NineSliceLoader;

impl AssetLoader for NineSliceLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let image = Image::from_buffer(bytes, ImageType::Extension(ext), default(), true)?
                .try_into_dynamic()?;

            let width = image.width() - 2;
            let height = image.height() - 2;

            let mut margins = Rect {
                min: Vec2::new(width as f32, height as f32),
                max: Vec2::new(0.0, 0.0),
            };

            let mut content = Rect {
                min: Vec2::new(width as f32, height as f32),
                max: Vec2::new(0.0, 0.0),
            };

            let Some(image_rgba8) = image.as_rgba8() else {
                panic!();
            };

            for x in 0..width {
                let begin = x as f32;
                let end = (x + 1) as f32;

                if image_rgba8[(x + 1, 0)][3] > 128 {
                    margins.min.x = margins.min.x.min(begin);
                    margins.max.x = margins.max.x.max(end);
                }

                if image_rgba8[(x + 1, height + 1)][3] > 128 {
                    content.min.x = content.min.x.min(begin);
                    content.max.x = content.max.x.max(end);
                }
            }

            margins.max.x = width as f32 - margins.max.x;
            content.max.x = width as f32 - margins.max.x;

            for y in 0..height {
                let begin = y as f32;
                let end = (y + 1) as f32;

                if image_rgba8[(0, y + 1)][3] > 128 {
                    margins.min.y = margins.min.y.min(begin);
                    margins.max.y = margins.max.y.max(end);
                }

                if image_rgba8[(width + 1, y + 1)][3] > 128 {
                    content.min.y = content.min.y.min(begin);
                    content.max.y = content.max.y.max(end);
                }
            }

            margins.max.y = width as f32 - margins.max.y;
            content.max.y = width as f32 - margins.max.y;

            let image = load_context.set_labeled_asset(
                "Image",
                LoadedAsset::new(Image::from_dynamic(
                    image.crop_imm(1, 1, width, height),
                    true,
                )),
            );

            load_context.set_default_asset(LoadedAsset::new(NineSlice {
                image,
                margins,
                content,
            }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["9.png", "9.jpg"]
    }
}
