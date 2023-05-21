use colored::Colorize;
use debug_print::debug_println;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use uuid::Uuid;

use crate::detection;
use crate::image_processor;

pub fn cli() {
    println!("{}", "Enter path to jpg:".bold());

    loop {
        let mut path = String::new();

        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read line")
            .to_string();

        let path = path.replace('\n', "");
        let recognition_results = image_processor::recognize_faces(&path);

        for (face_uuid, &ref recognition) in recognition_results.iter() {
            let mut person_options: Vec<&crate::db::person::Person> = Vec::new();
            for mtch in recognition.matches.iter() {
                match &mtch.0 {
                    None => {}
                    Some(person) => {
                        if !person_options.contains(&person) {
                            person_options.push(person)
                        }
                    }
                }
            }

            if person_options.len() > 0 {
                println!("{}", "is it?".bold());
                for (i, person) in person_options.iter().enumerate() {
                    println!("{}) {}", &i, &person.name);
                }
                println!("{}", "Enter number of one of the options".bold());
                println!("{}", "or".bold());
            }
            println!("{}", "enter person name".bold());

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            let recognized_person_uuid = match answer.trim().parse::<usize>() {
                Ok(num) => person_options[num].uuid.clone(),
                Err(_) => {
                    debug_println!("create new person");

                    let name = answer.to_string().replace('\n', "");

                    if name == "" {
                        panic!("Name must be given")
                    }

                    crate::db::person::Person::create(&Uuid::new_v4().to_string(), &name).uuid
                }
            };

            let moderated = (&"moderated".to_string(), &"true".to_string());
            let person_uuid = (&"person_uuid".to_string(), &recognized_person_uuid);
            let update_params = vec![moderated, person_uuid];
            crate::db::face::Face::update(&face_uuid, update_params);
        }
        break;
    }
    println!("{}", "See ya =)".green().bold());
}

pub fn train(dir: &String) {
    let mut people: HashMap<String, String> = HashMap::new();
    let path = Path::new(dir);
    let photos = files_list(path);

    let person_name_regex = Regex::new(r"(?:.*\/)?([a-z_]+)\/.*$").unwrap();
    for item in photos.iter() {
        let caps = person_name_regex.captures(item).unwrap();
        let name = match caps.get(1) {
            Some(mtch) => mtch.as_str().to_string(),
            None => panic!("Name of person is not found in path of the image"),
        };

        println!("{}", item.yellow());
        let photo = match detection::call(&item) {
            Err(error) => panic!("Face detection failed: {:?}", error),
            Ok(photo) => photo,
        };

        let mut recognized_person_uuid = "".to_string();
        if people.contains_key(&name) {
            recognized_person_uuid = people.get(&name).unwrap().to_string();
        } else {
            recognized_person_uuid =
                crate::db::person::Person::create(&Uuid::new_v4().to_string(), &name).uuid;
            people.insert(name, recognized_person_uuid.clone());
        }

        crate::db::photo::Photo::save_detection_result(&photo);

        let face_uuid = &photo.faces[0].uuid;
        let moderated = (&"moderated".to_string(), &"true".to_string());
        let person_uuid = (&"person_uuid".to_string(), &recognized_person_uuid);
        let update_params = vec![moderated, person_uuid];
        crate::db::face::Face::update(&face_uuid, update_params);
    }
}

fn files_list(dir: &Path) -> Vec<String> {
    let mut result = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.append(&mut files_list(&path));
            } else {
                result.push(path.to_str().unwrap().to_string());
            }
        }
    }
    result
}
