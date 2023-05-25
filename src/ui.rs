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
pub const PADDING: f32 = 15.0;

pub fn ui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(540., 800.)),
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

// split into component
// each should content its own data and allow to acess it by api
// example PeopleList should contain vec<Person>, person_form, person_search. Clients should be able read only   
struct VsUi {
    photos: Vec<UIPhoto>,
    people: Vec<crate::db::person::Person>,
    pub photos_rx: Option<mpsc::Receiver<UIPhoto>>,
    show_new_person_form: bool,
    show_recognition_form: bool,
    new_person_name: String,
    image_picked_path: String,
    recognition_result: Option<crate::image_processor::ProcessingResult>,
    edit_photo_uuid: Option<String>,
    current_photo_image: Option<RetainedImage>,
}

impl Default for VsUi {
    fn default() -> Self {
        Self {
            photos: Vec::new(),
            people: Vec::new(),
            photos_rx: None,
            show_new_person_form: false,
            show_recognition_form: false,
            new_person_name: String::from(""),
            image_picked_path: String::from(""),
            recognition_result: None,
            edit_photo_uuid: None,
            current_photo_image: None
        }
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
        self.photo_form(ctx);
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

    fn person_by_uuid(&self, uuid: &String) -> Option<&crate::db::person::Person> {
        self.people
            .iter()
            .find(|person| &person.uuid == uuid)
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
                    if recognize_btn.clicked() {
                        self.show_recognition_form = true;
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn photos_list(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for photo in self.photos.iter() {
                        let button = self.view_photo_list_item(ctx, ui, photo);
                        if button.clicked() { 
                            self.edit_photo_uuid = Some(photo.data.uuid.clone());
                        };
                        ui.add_space(PADDING);
                    }
                });
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
                        self.people.push(crate::db::person::Person::create(
                            &Uuid::new_v4().to_string(),
                            &self.new_person_name,
                        ));
                        self.new_person_name = String::from("");
                        self.show_new_person_form = false;
                    }
                });
            });
    }

    fn recognition_form(&mut self, ctx: &egui::Context) {
        if !self.show_recognition_form { return }
        egui::Window::new("Recognize faces")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let path_label = ui.label("Path to image: ");
                    ui.text_edit_singleline(&mut self.image_picked_path)
                        .labelled_by(path_label.id);
                });

                if let Some(result) = &self.recognition_result {
                    ui.label(RichText::new("Face differences:").strong());
                    if result.display_data.len() == 0 {
                        ui.label("no matches");
                    }
                    for (name, coef) in result.display_data.iter() {
                        ui.label(&format!("{} -> {}", name, coef));
                    }
                    let ui_photo = self.photos
                                       .iter()
                                       .find(|photo| photo.data.uuid == result.photo.uuid)
                                       .unwrap();

                    let button = self.view_photo_list_item(ctx, ui, ui_photo);
                    if button.clicked() { 
                        self.edit_photo_uuid = Some(ui_photo.data.uuid.clone());
                    };
                }

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.recognition_result = None;
                        self.new_person_name = String::from("");
                        self.show_recognition_form = false;
                    }

                    if ui.button("Recognize").clicked() {
                        let recognition_result = crate::image_processor::call(&self.image_picked_path);
                        let uuid = &recognition_result.photo.uuid;
                        let photo = crate::db::photo::Photo::find(&uuid);
                        let ui_photo = UIPhoto {
                            texture: read_image(&photo.uuid, "thumb.jpg"),
                            faces: photo.faces(),
                            data: photo,
                            list_item_size: [100.0, 80.0].into(),
                        };
                        self.photos.push(ui_photo);
                        self.recognition_result = Some(recognition_result);
                    }
                });
            });
    }

    fn view_photo_list_item(&self, ctx: &egui::Context, ui: &mut eframe::egui::Ui, photo: &UIPhoto) -> egui::Response {
        let interactive_element = ui.add(egui::ImageButton::new(
            photo.texture.texture_id(ctx),
            photo.texture.size_vec2(),
            // [100.0, 80.0]
        ));

        for face in photo.faces.iter() {
            if let Some(person) = self.person_by_uuid(&face.person_uuid){
                ui.label(RichText::new(&person.name).strong());
            }
        }
        interactive_element
    }

    fn photo_form(&mut self, ctx: &egui::Context) {
        let photo_uuid = match &self.edit_photo_uuid {
            Some(uuid) => uuid.clone(),
            None => return
        };

        egui::Window::new("View photo")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let texture: &RetainedImage = self.current_photo_image.get_or_insert_with(|| {
                    read_image(&photo_uuid, "original.jpg")
                });

                let [original_width, original_height] = texture.size();
                let dimensions = crate::storage::resize_to_fit(
                    &540,
                    &400,
                    &(original_width as u32),
                    &(original_height as u32),
                );

                ui.add(egui::Image::new(
                    texture.texture_id(ctx),
                    [dimensions.width as f32, dimensions.height as f32]
                ));

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.edit_photo_uuid = None;
                        self.current_photo_image = None;
                    }

                    if ui.button("Save").clicked() {
                        self.edit_photo_uuid = None;
                        self.current_photo_image = None;
                        // let recognition_result = crate::image_processor::call(&self.image_picked_path);
                        // let uuid = &recognition_result.photo.uuid;
                        // let photo = crate::db::photo::Photo::find(&uuid);
                        // let ui_photo = UIPhoto {
                        //     texture: read_image(&photo.uuid, "thumb.jpg"),
                        //     faces: photo.faces(),
                        //     data: photo,
                        //     list_item_size: [100.0, 80.0].into(),
                        // };
                        // self.photos.push(ui_photo);
                        // self.recognition_result = Some(recognition_result);
                    }
                });

            });

        // let img_button = egui::ImageButton::new(
        //     photo.texture.texture_id(ctx),
        //     photo.texture.size_vec2(),
        //     // [100.0, 80.0]
        // );

        // ui.add(img_button);

        // for face in photo.faces.iter() {
        //     if let Some(person) = self.person_by_uuid(&face.person_uuid){
        //         ui.label(RichText::new(&person.name).size(10.0).strong());
        //     }
        // }
    }
}

pub fn read_image(uuid: &String, name: &str) -> RetainedImage {
    println!("ui is reading image...");

    let mut buffer = vec![];

    File::open(format!("{}/{}/{}", storage::IMAGES_DIR, uuid, name))
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    RetainedImage::from_image_bytes(name, &buffer[..]).unwrap()
}
