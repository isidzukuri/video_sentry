use crate::detection;
use crate::recognition::{self, Data};
use std::collections::HashMap;

const MAX_DISTANCE: f64 = 0.6;

pub struct ProcessingResult {
    pub photo: detection::photo::Photo,
    pub display_data: Vec<(String, f64)>,
    pub face_matches: HashMap<String, crate::recognition::Data>,
}

pub fn call(path: &String) -> ProcessingResult {
    let mut result: Vec<(String, f64)> = Vec::new();
    let mut recognition_results = recognize_faces(&path);

    for (face_uuid, &ref recognition) in recognition_results.face_matches.iter() {
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
    recognition_results.display_data = result;
    recognition_results
}

pub fn recognize_faces(path: &String) -> ProcessingResult {
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

    ProcessingResult {
        photo: photo,
        face_matches: recognition_results,
        display_data: Vec::new(),
    }
}
