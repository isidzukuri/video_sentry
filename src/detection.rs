use dlib_face_recognition::FaceDetectorCnn;
use dlib_face_recognition::*;
use image;
use image::*;
use std::error::Error;

use crate::detection::face_image::FaceImage;
use crate::detection::photo::Photo;
use crate::storage;

pub mod face_image;
pub mod photo;

pub fn call(path: &String) -> Result<Photo, Box<dyn Error>> {
    let mut image = image::open(path).unwrap();
    let mut photo = Photo::new();
    let folder_path: String = format!("{}/{}/", storage::IMAGES_DIR, photo.uuid);

    storage::save_original_image(&folder_path, &image).expect("original image cannot be saved");
    storage::save_thumbnail(&folder_path, &image).expect("original image cannot be saved");

    let face_locations = detect_faces(&image);

    if face_locations.len() == 0 {
        println!("Faces are not detected")
    }

    for rect in face_locations.iter() {
        match crop_face(rect, &mut image, &folder_path) {
            Err(error) => panic!("Face cannot be cropped: {:?}", error),
            Ok(face_image) => photo.add_face(face_image),
        }
    }

    photo.push_img(image);
    measure_faces(&mut photo);
    Ok(photo)
}

fn crop_face(
    rect: &Rectangle,
    image: &mut DynamicImage,
    folder_path: &String,
) -> Result<FaceImage, ImageError> {
    let width: u32 = (rect.right - rect.left).try_into().unwrap();
    let height: u32 = (rect.bottom - rect.top).try_into().unwrap();
    let sub_image = image::imageops::crop(image, rect.left as u32, rect.top as u32, width, height);

    let mut face_image = FaceImage::new();
    let path = format!("{}/{}.jpg", folder_path, face_image.uuid);
    match sub_image.to_image().save(path.clone()) {
        Err(error) => Err(error),
        Ok(_file) => {
            face_image.store_face_location(*rect);
            Ok(face_image)
        }
    }
}

fn detect_faces(image: &DynamicImage) -> FaceLocations {
    let matrix = ImageMatrix::from_image(&image.to_rgb8());

    let Ok(cnn_detector) = FaceDetectorCnn::default() else {
        panic!("Unable to load cnn face detector!");
    };

    cnn_detector.face_locations(&matrix)
}

fn measure_faces(photo: &mut Photo) {
    let Ok(landmarks) = LandmarkPredictor::default() else {
        panic!("Error loading Landmark Predictor.");
    };

    let Ok(face_encoder) = FaceEncoderNetwork::default() else {
        panic!("Error loading Face Encoder.");
    };

    let matrix = ImageMatrix::from_image(&photo.image.as_ref().unwrap().to_rgb8());

    for face in photo.faces.iter_mut() {
        let landmarks = landmarks.face_landmarks(&matrix, &face.face_location.unwrap());

        let encodings = face_encoder.get_face_encodings(&matrix, &[landmarks], 0); // -> FaceEncodings

        let face_measurements = encodings.first().unwrap();

        face.store_measurements(face_measurements.as_ref().to_vec());
    }
}
