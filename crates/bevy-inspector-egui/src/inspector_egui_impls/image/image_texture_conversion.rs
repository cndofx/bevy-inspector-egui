use bevy_render::{
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::{Image, TextureFormatPixelInfo},
};
use image::{DynamicImage, ImageBuffer};

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
            DynamicImage::ImageLuma8(ImageBuffer::from_raw(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                image.data.clone(),
            )?),
            false,
        ),
        TextureFormat::R32Float => ({
            use image::Luma;
            let f32_data = convert_bytes_to_f32(&image.data);
            let width = image.texture_descriptor.size.width as u32;
            let height = image.texture_descriptor.size.height as u32;
            
            // Create a new ImageBuffer for Luma<u8> (grayscale)
            let mut imgbuf = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width, height);
            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                let data_index = (y * width + x) as usize;
                // Normalize and convert the floating-point value to a u8 grayscale value.
                // Adjust normalization as needed for your specific data range.
                let normalized_value = normalize_f32_to_u8(f32_data[data_index]);
                *pixel = Luma([normalized_value]);
            }
            image::DynamicImage::ImageLuma8(imgbuf)
               
            },
            false,
        ),
        _ => return None,
    };
    Some((image, is_srgb))
}

fn convert_bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
    // Ensure the bytes length is a multiple of 4 for safe conversion to f32
    assert!(bytes.len() % 4 == 0, "Data length is not aligned for f32 conversion");

    // SAFETY: We've ensured alignment and size correctness above.
    // The caller must ensure that the byte slice's lifetime is valid for the conversion.
    unsafe {
        std::slice::from_raw_parts(bytes.as_ptr() as *const f32, bytes.len() / 4).to_vec()
    }
}

fn normalize_f32_to_u8(value: f32) -> u8 {
    // Example: clamp value between 0.0 and 1.0 and scale to 0-255
    (value.clamp(0.0, 1.0) * 255.0) as u8
}