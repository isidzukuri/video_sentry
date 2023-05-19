use colored::*;
use debug_print::debug_println;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use uuid::Uuid;
use colored::Colorize;

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

pub fn train() {
    let mut people: HashMap<String, String> = HashMap::new();

    // photos with exact one face
    let photos: [String; 16] = [
        "/home/isidzukuri/rust_projects/video_sentry/people/a/1.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/b/1.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/b/2.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/b/3.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/b/4.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/c/1.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/c/2.jpeg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/c/3.jpeg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/c/4.jpeg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/c/5.jpeg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/d/1.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/d/2.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/d/3.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/d/4.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/armas/1.jpg".to_string(),
        "/home/isidzukuri/rust_projects/video_sentry/people/marion_cotillard/1.jpg".to_string(),
    ];

    for item in photos.iter() {
        println!("{}", item.yellow());
        let photo = match detection::call(&item) {
            Err(error) => panic!("Face detection failed: {:?}", error),
            Ok(photo) => photo,
        };

        let re = Regex::new(r".*\/([a-z_]+)\/.*$").unwrap();
        let caps = re.captures(item).unwrap();
        let name = caps.get(1).map_or("", |m| m.as_str()).to_string();

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
