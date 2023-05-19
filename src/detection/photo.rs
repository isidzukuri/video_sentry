use crate::detection::face_image::FaceImage;
use debug_print::debug_println;
use image::DynamicImage;
use std::fmt;
use uuid::Uuid;

pub struct Photo {
    pub uuid: String,
    pub face_detected: bool,
    pub faces: Vec<FaceImage>,
    pub image: Option<DynamicImage>,
}

impl Photo {
    pub fn new() -> Photo {
        Photo {
            uuid: Uuid::new_v4().to_string(),
            faces: Vec::new(),
            face_detected: false,
            image: None,
        }
    }

    pub fn add_face(&mut self, face_image: FaceImage) {
        self.faces.push(face_image);
        self.face_detected = true;
    }

    pub fn push_img(&mut self, image: DynamicImage) {
        self.image = Some(image);
    }
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#################################\r\n").unwrap();
        write!(f, "{}\r\n", self.uuid).unwrap();
        write!(f, "face detected: {}\r\n", self.face_detected).unwrap();
        for face in self.faces.iter() {
            debug_println!("{}", face);
        }
        write!(f, "#################################")
    }
}
