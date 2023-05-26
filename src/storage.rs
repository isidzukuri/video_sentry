use egui_extras::image::RetainedImage;
use image;
use image::*;
use std::fs;
use std::fs::File;
use std::io::Read;

pub const IMAGES_DIR: &str = "storage/images/";
pub const THUMB_DIMENSIONS: AreaDimensions = AreaDimensions {
    width: 400,
    height: 120,
};

pub struct AreaDimensions {
    pub width: u32,
    pub height: u32,
}

pub fn save_original_image(folder_path: &String, image: &DynamicImage) -> Result<(), ImageError> {
    match fs::create_dir_all(folder_path) {
        Err(error) => panic!("Storage folder cannot be created: {:?}", error),
        Ok(_) => image.save(format!("{}/original.jpg", folder_path)),
    }
}

pub fn save_thumbnail(folder_path: &String, image: &DynamicImage) -> Result<(), ImageError> {
    let (current_width, current_height) = image.dimensions();
    let dimensions = resize_to_fit(
        &THUMB_DIMENSIONS.width,
        &THUMB_DIMENSIONS.height,
        &current_width,
        &current_height,
    );

    match fs::create_dir_all(folder_path) {
        Err(error) => panic!("Storage folder cannot be created: {:?}", error),
        Ok(_) => image::imageops::thumbnail(image, dimensions.width, dimensions.height)
            .save(format!("{}/thumb.jpg", folder_path)),
    }
}

pub fn resize_to_fit(
    area_width: &u32,
    area_height: &u32,
    image_width: &u32,
    image_height: &u32,
) -> AreaDimensions {
    let ratio = calculate_ratio(area_width, area_height, image_width, image_height);

    AreaDimensions {
        width: (*image_width as f64 * ratio) as u32,
        height: (*image_height as f64 * ratio) as u32,
    }
}

pub fn calculate_ratio(
    area_width: &u32,
    area_height: &u32,
    image_width: &u32,
    image_height: &u32,
) -> f64 {
    let (area_size, image_size);

    if area_height * image_width < area_width * image_height {
        area_size = area_height;
        image_size = image_height;
    } else {
        area_size = area_width;
        image_size = image_width;
    }

    let ratio = *area_size as f64 / *image_size as f64;

    ratio
}

pub fn read_image_for_ui(uuid: &String, name: &str) -> RetainedImage {
    println!("ui is reading image...");

    let mut buffer = vec![];

    File::open(format!("{}/{}/{}", IMAGES_DIR, uuid, name))
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    RetainedImage::from_image_bytes(name, &buffer[..]).unwrap()
}
