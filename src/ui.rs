use std::fs::File;
use std::sync::mpsc;
use std::thread;
use uuid::Uuid;

use eframe::egui;
use egui::Color32;
use egui::Vec2;

use egui_extras::image::RetainedImage;
// use egui_extras::RetainedImage;
use std::io::Read;

// use egui::FontDefinitions;
// use egui::epi::App;
use egui::{FontFamily, FontId, RichText, TextStyle};

use crate::db::photo::Photo;
use crate::storage;

// const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
// const GREEN: Color32 = Color32::from_rgb(37, 184, 76);
// const RED: Color32 = Color32::from_rgb(174, 78, 37);

pub fn ui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(540., 960.)),
        ..Default::default()
    };

    eframe::run_native(
        "Video Sentry",
        options,
        Box::new(|context| Box::new(VsUi::new(context))),
    )
}

struct UIPhoto {
    texture: RetainedImage,
    data: Photo,
    faces: Vec<crate::db::face::Face>,
    list_item_size: Vec2,
}

struct VsUi {
    photos: Vec<UIPhoto>,
    people: Vec<crate::db::person::Person>,
    pub photos_rx: Option<mpsc::Receiver<UIPhoto>>,
    show_new_person_form: bool,
    new_person_name: String,
}

impl Default for VsUi {
    fn default() -> Self {
        Self {
            photos: Vec::new(),
            people: Vec::new(),
            photos_rx: None,
            show_new_person_form: false,
            new_person_name: String::from(""),
        }
    }
}

impl VsUi {
    fn new(_context: &eframe::CreationContext<'_>) -> Self {
        let (mut photos_tx, photos_rx) = mpsc::channel();
        let instance = Self {
            people: crate::db::person::Person::all(),
            photos_rx: Some(photos_rx),
            ..Default::default()
        };

        thread::spawn(move || {
            Self::fetch_photos(&mut photos_tx);
        });

        instance
    }

    fn fetch_photos(photos_tx: &mut std::sync::mpsc::Sender<UIPhoto>) {
        for photo in Photo::all().iter() {
            let ui_photo = UIPhoto {
                texture: read_image(&photo.uuid, "thumb.jpg"),
                data: photo.clone(),
                faces: photo.faces(),
                list_item_size: [100.0, 80.0].into(),
            };

            if let Err(e) = photos_tx.send(ui_photo) {
                panic!("Error sending news data: {}", e);
            }
        }
    }

    fn preload_photos(&mut self) {
        if let Some(rx) = &self.photos_rx {
            match rx.try_recv() {
                Ok(ui_photo) => {
                    self.photos.push(ui_photo);
                }
                Err(_e) => {
                    // println!("Error receiving msg: {}", e);
                }
            }
        }
    }

    fn render_photos(&self, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
        for photo in self.photos.iter() {
            ui.add(egui::ImageButton::new(
                photo.texture.texture_id(ctx),
                photo.texture.size_vec2(),
                // [100.0, 80.0]
            ));

            for face in photo.faces.iter() {
                let person = self.person_by_uuid(&face.person_uuid);
                ui.label(RichText::new(&person.name).size(10.0).strong());
            }
            // name
            // status
            // approve button
        }
    }

    fn person_by_uuid(&self, uuid: &String) -> &crate::db::person::Person {
        self.people
            .iter()
            .find(|person| &person.uuid == uuid)
            .unwrap()
    }

    fn top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let recognize_btn = ui.add(egui::Button::new("Recognize"));
                    let add_person_button = ui.add(egui::Button::new("Add Person"));

                    if add_person_button.clicked() {
                        self.show_new_person_form = true;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn photos_list(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_photos(ui, ctx);
            });
        });
    }

    fn new_person_form(&mut self, ctx: &egui::Context) {
        if !self.show_new_person_form { return }
        egui::Window::new("Add new person")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let name_label = ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.new_person_name)
                        .labelled_by(name_label.id);
                });

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.new_person_name = String::from("");
                        self.show_new_person_form = false;
                    }

                    if ui.button("Save").clicked() {
                        crate::db::person::Person::create(
                            &Uuid::new_v4().to_string(),
                            &self.new_person_name,
                        );
                        self.new_person_name = String::from("");
                        self.show_new_person_form = false;
                    }
                });
            });
    }

    fn recognition_form(&mut self, ctx: &egui::Context) {
        // if !self.show_new_person_form { return }
        // egui::Window::new("Add new person")
        //     .collapsible(false)
        //     .resizable(false)
        //     .show(ctx, |ui| {
        //         ui.horizontal(|ui| {
        //             let name_label = ui.label("Name: ");
        //             ui.text_edit_singleline(&mut self.new_person_name)
        //                 .labelled_by(name_label.id);
        //         });

        //         ui.horizontal(|ui| {
        //             if ui.button("Cancel").clicked() {
        //                 self.new_person_name = String::from("");
        //                 self.show_new_person_form = false;
        //             }

        //             if ui.button("Save").clicked() {
        //                 crate::db::person::Person::create(
        //                     &Uuid::new_v4().to_string(),
        //                     &self.new_person_name,
        //                 );
        //                 self.new_person_name = String::from("");
        //                 self.show_new_person_form = false;
        //             }
        //         });
        //     });
    }
}

impl eframe::App for VsUi {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.preload_photos();

        self.top_panel(ctx);
        self.photos_list(ctx);
        self.new_person_form(ctx);
        self.recognition_form(ctx);
    }
}

pub fn read_image(uuid: &String, name: &str) -> RetainedImage {
    let mut buffer = vec![];

    File::open(format!("{}/{}/{}", storage::IMAGES_DIR, uuid, name))
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    RetainedImage::from_image_bytes(name, &buffer[..]).unwrap()
}
