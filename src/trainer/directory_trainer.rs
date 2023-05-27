use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use crate::detection;
use crate::detection::photo::Photo;

pub struct DirectoryTrainer {
    pub dir: String,
    pub people: HashMap<String, String>,
}

impl DirectoryTrainer {
    pub fn new(dir: String) -> DirectoryTrainer {
        Self {
            dir: dir,
            people: HashMap::new(),
        }
    }

    pub fn call(&mut self) {
        let path = Path::new(&self.dir);
        let photos = Self::files_list(path);

        for file_path in photos.iter() {
            self.process_photo(file_path);
        }
    }

    fn files_list(dir: &Path) -> Vec<String> {
        let mut result = Vec::new();

        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    result.append(&mut Self::files_list(&path));
                } else {
                    result.push(path.to_str().unwrap().to_string());
                }
            }
        }
        result
    }

    fn process_photo(&mut self, file_path: &String) {
        println!("{}", file_path.yellow());

        let name = Self::parse_name(&file_path);
        let photo = Self::analyze_photo(file_path);

        crate::db::photo::Photo::save_detection_result(&photo);

        let recognized_person_uuid = self.get_person_uuid(name);
        let face_uuid = &photo.faces[0].uuid;
        crate::db::face::Face::moderate_person(&face_uuid, &recognized_person_uuid);
    }

    fn parse_name(file_path: &String) -> String {
        let caps = Self::person_name_regex().captures(file_path).unwrap();
        match caps.get(1) {
            Some(mtch) => mtch.as_str().to_string(),
            None => panic!("Name of person is not found in path of the image"),
        }
    }

    // todo memoize or make constant
    fn person_name_regex() -> Regex {
        Regex::new(r"(?:.*\/)?([a-z_]+)\/.*$").unwrap()
    }

    fn analyze_photo(file_path: &String) -> Photo {
        match detection::call(&file_path) {
            Err(error) => panic!("Face detection failed: {:?}", error),
            Ok(photo) => photo,
        }
    }

    fn get_person_uuid(&mut self, name: String) -> String {
        match self.people.contains_key(&name) {
            true => self.people.get(&name).unwrap().to_string(),
            false => {
                let recognized_person_uuid =
                    crate::db::person::Person::create(&Uuid::new_v4().to_string(), &name).uuid;
                self.people.insert(name, recognized_person_uuid.clone());
                recognized_person_uuid
            }
        }
    }
}
