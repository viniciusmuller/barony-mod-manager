use iced::{
    image::Handle,
    window::{self, Icon},
};
use image::{
    imageops::{resize as lib_resize, FilterType::Triangle},
    DynamicImage, ImageBuffer, Rgba,
};

type CrateImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

static APP_LOGO: &[u8] = include_bytes!("../resources/img/logo.png");
static APP_LOGO_WIDTH: u32 = 160;

pub fn resize(image: &DynamicImage, width: u32, height: u32) -> CrateImage {
    lib_resize(image, width, height, Triangle)
}

pub fn to_handle(image: &CrateImage) -> Handle {
    Handle::from_pixels(image.width(), image.height(), image.to_vec())
}

pub fn build_app_logo() -> Result<Icon, window::icon::Error> {
    let img = image::load_from_memory(APP_LOGO).unwrap().to_rgba8();
    Icon::from_rgba(img.to_vec(), APP_LOGO_WIDTH, APP_LOGO_WIDTH)
}
