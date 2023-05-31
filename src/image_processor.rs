use crate::detection;
use crate::recognition;
use std::collections::HashMap;
use std::error::Error;

const MAX_DISTANCE: f64 = 0.6;

pub struct ProcessingResult {
    pub photo: detection::photo::Photo,
    pub display_data: Vec<(String, f64)>,
    pub face_matches: HashMap<String, crate::recognition::Data>,
}

pub fn call(path: &String) -> Option<ProcessingResult> {
    match recognize_faces(&path) {
        Err(error) => {
            println!("Face detection failed: {:?}", error);
            None
        },
        Ok(data) => {
            let mut result: Vec<(String, f64)> = Vec::new();
            let mut recognition_results = data;

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
            Some(recognition_results)
        },
    }
}

pub fn recognize_faces(path: &String) -> Result<ProcessingResult, Box<dyn Error>> {
    match detection::call(&path) {
        Err(error) => Err(error),
        Ok(photo) => {
            let mut recognition_results = HashMap::new();
            crate::db::photo::Photo::save_detection_result(&photo);

            for face in photo.faces.iter() {
                let result = recognition::find_matches(&face.uuid);

                recognition_results.insert(face.uuid.clone(), result);
            }

            Ok(ProcessingResult {
                photo: photo,
                face_matches: recognition_results,
                display_data: Vec::new(),
            })        
        },
    }
}
