use crate::detection::Rectangle;
use std::fmt;
use uuid::Uuid;

pub struct FaceImage {
    pub uuid: String,
    pub face_location: Option<Rectangle>,
    pub measurements: Vec<f64>,
}

impl FaceImage {
    pub fn new() -> FaceImage {
        FaceImage {
            uuid: Uuid::new_v4().to_string(),
            face_location: None,
            measurements: Vec::new(),
        }
    }

    pub fn store_face_location(&mut self, location: Rectangle) {
        self.face_location = Some(location);
    }

    pub fn store_measurements(&mut self, measurements: Vec<f64>) {
        self.measurements = measurements;
    }
}

impl fmt::Display for FaceImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "face: {}\r\n", self.uuid).unwrap();
        write!(f, "location: {:?}\r\n", self.face_location).unwrap();
        write!(f, "measurements: {:?}\r\n", self.measurements)
    }
}
