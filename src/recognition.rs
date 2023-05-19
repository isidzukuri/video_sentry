use debug_print::debug_println;
use dlib_face_recognition::*;
use std::fmt;

pub struct Data {
    pub matches: Vec<(Option<crate::db::person::Person>, f64)>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    pub fn order_matches<T>(matches: &mut Vec<(T, f64)>) {
        matches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }
}

pub fn find_matches(uuid: &String) -> Data {
    let left_face = crate::db::face::Face::find(&uuid);
    let mut result = Data::new();

    let left_encoding = face_encoding(&left_face);

    for right_face in crate::db::face::Face::where_all(&"moderated = 'true'".to_string()) {
        let right_encoding = face_encoding(&right_face);
        let distance = left_encoding.distance(&right_encoding);
        let person = if right_face.person_uuid != "" {
            Some(crate::db::person::Person::find(&right_face.person_uuid))
        } else {
            None
        };
        result.matches.push((person, distance));
    }

    Data::order_matches(&mut result.matches);
    debug_println!("{}", result);
    result
}

fn face_encoding(face: &crate::db::face::Face) -> FaceEncoding {
    FaceEncoding::from_vec(&face.deserialize_measurements()).unwrap()
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "..................................\r\n").unwrap();
        write!(f, "Face comparison results: \r\n",).unwrap();
        for item in self.matches.iter() {
            println!("{} -> {:?}", item.1, item.0);
        }
        write!(f, "..................................\r\n")
    }
}

// pub fn compare_faces(uuid: &String) {
//     let left_face = crate::db::face::Face::find(&uuid);
//     let left_encoding = FaceEncoding::from_vec(&left_face.deserialize_measurements()).unwrap();

//     for right_face in crate::db::face::Face::all() {
//         let right_encoding =
//             FaceEncoding::from_vec(&right_face.deserialize_measurements()).unwrap();

//         let distance = left_encoding.distance(&right_encoding);

//         println!("{} vs {} -> {:?}", right_face.id, left_face.id, distance);
//         println!("**********************************");
//     }
// }
