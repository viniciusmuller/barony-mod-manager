use iced::image::Handle;
use image::{
    imageops::{resize as lib_resize, FilterType::Triangle},
    DynamicImage, ImageBuffer, Rgba,
};

type CrateImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn resize(image: &DynamicImage, width: u32, height: u32) -> CrateImage {
    lib_resize(image, width, height, Triangle)
}

pub fn to_handle(image: &CrateImage) -> Handle {
    Handle::from_pixels(image.width(), image.height(), image.to_vec())
}
