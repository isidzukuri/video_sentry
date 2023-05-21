use colored::Colorize;
use debug_print::debug_println;
use std::io;
use uuid::Uuid;

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