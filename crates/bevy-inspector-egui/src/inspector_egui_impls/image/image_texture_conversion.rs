use bevy_log::warn;
use bevy_render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::{Image, TextureFormatPixelInfo},
};
use image::{DynamicImage, ImageBuffer, Rgba};

/// Converts a [`DynamicImage`] to an [`Image`].
pub fn from_dynamic(dyn_img: DynamicImage, is_srgb: bool) -> Image {
    use bevy_core::cast_slice;
    let width;
    let height;

    let data: Vec<u8>;
    let format: TextureFormat;

    match dyn_img {
        DynamicImage::ImageLuma8(i) => {
            let i = DynamicImage::ImageLuma8(i).into_rgba8();
            width = i.width();
            height = i.height();
            format = if is_srgb {
                TextureFormat::Rgba8UnormSrgb
            } else {
                TextureFormat::Rgba8Unorm
            };

            data = i.into_raw();
        }
        DynamicImage::ImageLumaA8(i) => {
            let i = DynamicImage::ImageLumaA8(i).into_rgba8();
            width = i.width();
            height = i.height();
            format = if is_srgb {
                TextureFormat::Rgba8UnormSrgb
            } else {
                TextureFormat::Rgba8Unorm
            };

            data = i.into_raw();
        }
        DynamicImage::ImageRgb8(i) => {
            let i = DynamicImage::ImageRgb8(i).into_rgba8();
            width = i.width();
            height = i.height();
            format = if is_srgb {
                TextureFormat::Rgba8UnormSrgb
            } else {
                TextureFormat::Rgba8Unorm
            };

            data = i.into_raw();
        }
        DynamicImage::ImageRgba8(i) => {
            width = i.width();
            height = i.height();
            format = if is_srgb {
                TextureFormat::Rgba8UnormSrgb
            } else {
                TextureFormat::Rgba8Unorm
            };

            data = i.into_raw();
        }
        DynamicImage::ImageLuma16(i) => {
            width = i.width();
            height = i.height();
            format = TextureFormat::R16Uint;

            let raw_data = i.into_raw();

            data = cast_slice(&raw_data).to_owned();
        }
        DynamicImage::ImageLumaA16(i) => {
            width = i.width();
            height = i.height();
            format = TextureFormat::Rg16Uint;

            let raw_data = i.into_raw();

            data = cast_slice(&raw_data).to_owned();
        }
        DynamicImage::ImageRgb16(image) => {
            width = image.width();
            height = image.height();
            format = TextureFormat::Rgba16Uint;

            let mut local_data =
                Vec::with_capacity(width as usize * height as usize * format.pixel_size());

            for pixel in image.into_raw().chunks_exact(3) {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = u16::max_value();

                local_data.extend_from_slice(&r.to_ne_bytes());
                local_data.extend_from_slice(&g.to_ne_bytes());
                local_data.extend_from_slice(&b.to_ne_bytes());
                local_data.extend_from_slice(&a.to_ne_bytes());
            }

            data = local_data;
        }
        DynamicImage::ImageRgba16(i) => {
            width = i.width();
            height = i.height();
            format = TextureFormat::Rgba16Uint;

            let raw_data = i.into_raw();

            data = cast_slice(&raw_data).to_owned();
        }
        DynamicImage::ImageRgb32F(image) => {
            width = image.width();
            height = image.height();
            format = TextureFormat::Rgba32Float;

            let mut local_data =
                Vec::with_capacity(width as usize * height as usize * format.pixel_size());

            for pixel in image.into_raw().chunks_exact(3) {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = u16::max_value();

                local_data.extend_from_slice(&r.to_ne_bytes());
                local_data.extend_from_slice(&g.to_ne_bytes());
                local_data.extend_from_slice(&b.to_ne_bytes());
                local_data.extend_from_slice(&a.to_ne_bytes());
            }

            data = local_data;
        }
        DynamicImage::ImageRgba32F(image) => {
            width = image.width();
            height = image.height();
            format = TextureFormat::Rgba32Float;

            let raw_data = image.into_raw();

            data = cast_slice(&raw_data).to_owned();
        }
        // DynamicImage is now non exhaustive, catch future variants and convert them
        _ => {
            let image = dyn_img.into_rgba8();
            width = image.width();
            height = image.height();
            format = TextureFormat::Rgba8UnormSrgb;

            data = image.into_raw();
        }
    }

    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::default(),
    )
}

pub fn try_into_dynamic(image: &Image) -> Option<(DynamicImage, bool)> {
    let (image, is_srgb) = match image.texture_descriptor.format {
        TextureFormat::R8Unorm => (
            DynamicImage::ImageLuma8(ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )?),
            false,
        ),
        TextureFormat::Rg8Unorm => (
            DynamicImage::ImageLumaA8(ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )?),
            false,
        ),
        TextureFormat::Rgba8UnormSrgb => (
            DynamicImage::ImageRgba8(ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )?),
            true,
        ),
        TextureFormat::Rgba8Unorm => (
            DynamicImage::ImageRgba8(ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )?),
            false,
        ),
        TextureFormat::R8Uint => {
            let width = image.texture_descriptor.size.width;
            let height = image.texture_descriptor.size.height;

            let mut imgbuf = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);

            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                let data_index = (y * width + x) as usize;
                let value = image.data[data_index];
                *pixel = Rgba([value, value, value, 255]);
            }
            (DynamicImage::ImageRgba8(imgbuf), false)
        }
        TextureFormat::R32Float => {
            let f32_data: Vec<f32> = bevy_core::cast_slice(&image.data).to_owned();

            //let f32_data =  convert_bytes_to_f32(&image.data);
            let width = image.texture_descriptor.size.width;
            let height = image.texture_descriptor.size.height;

            let mut imgbuf = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);

            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                let data_index = (y * width + x) as usize;
                let value = f32_data[data_index];

                // Assign colors based on sign
                let scale = 100.0;
                let normalized_value = value.clamp(-scale, scale) / scale;
                let (green, red) = if normalized_value > 0.0 {
                    // Positive values: Map to red, scale intensity by normalized value
                    (normalized_value * 255.0, 0.0)
                } else {
                    // Negative values: Map to green, scale intensity by absolute value of normalized value
                    (0.0, normalized_value.abs() * 255.0)
                };

                *pixel = Rgba([red as u8, green as u8, 0, 255]);
            }
            (DynamicImage::ImageRgba8(imgbuf), false)
        }
        v @ _ => {
            // TODO: remove repeating error, but useful for now
            warn!("Unsupported texture format, {:?}", v);
            return None;
        }
    };
    Some((image, is_srgb))
}
