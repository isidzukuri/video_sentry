use crate::detection;
use crate::recognition::{self, Data};
use std::collections::HashMap;

const MAX_DISTANCE: f64 = 0.6;

pub fn call(path: &String) -> Vec<(String, f64)> {
    let mut result: Vec<(String, f64)> = Vec::new();
    let recognition_results = recognize_faces(&path);

    for (face_uuid, &ref recognition) in recognition_results.iter() {
        if recognition.matches.len() == 0 {
            continue;
        }

        let (Some(person), coef) = &recognition.matches[0] else { todo!() };

        if coef < &MAX_DISTANCE {
            crate::db::face::Face::update_person_uuid(&face_uuid, &person.uuid);
            result.push((person.name.clone(), *coef));
        }
    }
    println!("{:?}", result);
    result
}

pub fn recognize_faces(path: &String) -> HashMap<String, Data> {
    let mut recognition_results = HashMap::new();
    let photo = match detection::call(&path) {
        Err(error) => panic!("Face detection failed: {:?}", error),
        Ok(photo) => photo,
    };

    crate::db::photo::Photo::save_detection_result(&photo);

    for face in photo.faces.iter() {
        let result = recognition::find_matches(&face.uuid);

        recognition_results.insert(face.uuid.clone(), result);
    }

    recognition_results
}
