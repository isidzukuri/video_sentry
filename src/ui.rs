use std::sync::mpsc;
use std::thread;
use uuid::Uuid;

use eframe::egui;
use egui::RichText;
use egui_extras::image::RetainedImage;

use crate::db::photo::Photo;
use crate::storage;

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
}

impl UIPhoto {
    pub fn set_faces(&mut self, faces: Vec<crate::db::face::Face>) {
        self.faces = faces;
    }
}

struct FaceFormData {
    uuid: String,
    texture: RetainedImage,
    selected_person_option: (String, String),
}

// split into components
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
    faces_form_data: Vec<FaceFormData>,
    person_options: Vec<(String, String)>,
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
            current_photo_image: None,
            faces_form_data: Vec::new(),
            person_options: Vec::new(),
        }
    }
}

impl eframe::App for VsUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                texture: storage::read_image_for_ui(&photo.uuid, "thumb.jpg"),
                data: photo.clone(),
                faces: photo.faces(),
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
        self.people.iter().find(|person| &person.uuid == uuid)
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
            ui.vertical_centered(|ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for photo in self.photos.iter() {
                        let button = self.view_photo_list_item(ctx, ui, photo);
                        if button.clicked() {
                            self.current_photo_image =
                                Some(storage::read_image_for_ui(&photo.data.uuid, "original.jpg"));

                            for face in photo.faces.iter() {
                                let selected_person_option = match face.person() {
                                    Some(person) => (person.uuid, person.name),
                                    None => (String::from(""), String::from("")),
                                };

                                self.faces_form_data.push(FaceFormData {
                                    uuid: face.uuid.clone(),
                                    texture: storage::read_image_for_ui(
                                        &photo.data.uuid,
                                        format!("{}.jpg", face.uuid).as_str(),
                                    ),
                                    selected_person_option: selected_person_option,
                                });
                            }

                            for person in self.people.iter() {
                                self.person_options.push((
                                    person.uuid.clone().to_string(),
                                    person.name.clone().to_string(),
                                ));
                            }

                            self.edit_photo_uuid = Some(photo.data.uuid.clone());
                        };
                        ui.add_space(PADDING);
                    }
                });
            });
        });
    }

    fn new_person_form(&mut self, ctx: &egui::Context) {
        if !self.show_new_person_form {
            return;
        }
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
        if !self.show_recognition_form {
            return;
        }
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
                    let ui_photo = self
                        .photos
                        .iter()
                        .find(|photo| photo.data.uuid == result.photo.uuid)
                        .unwrap();

                    self.view_photo_list_item(ctx, ui, ui_photo);
                }

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.recognition_result = None;
                        self.new_person_name = String::from("");
                        self.show_recognition_form = false;
                    }

                    if ui.button("Recognize").clicked() {
                        let recognition_result =
                            crate::image_processor::call(&self.image_picked_path);
                        let uuid = &recognition_result.photo.uuid;
                        let photo = crate::db::photo::Photo::find(&uuid);
                        let ui_photo = UIPhoto {
                            texture: storage::read_image_for_ui(&photo.uuid, "thumb.jpg"),
                            faces: photo.faces(),
                            data: photo,
                        };
                        self.photos.push(ui_photo);
                        self.recognition_result = Some(recognition_result);
                    }
                });
            });
    }

    fn view_photo_list_item(
        &self,
        ctx: &egui::Context,
        ui: &mut eframe::egui::Ui,
        photo: &UIPhoto,
    ) -> egui::Response {
        let interactive_element = ui.add(egui::ImageButton::new(
            photo.texture.texture_id(ctx),
            photo.texture.size_vec2(),
            // [100.0, 80.0]
        ));

        for face in photo.faces.iter() {
            if let Some(person) = self.person_by_uuid(&face.person_uuid) {
                ui.label(RichText::new(&person.name).strong());
            }
        }
        interactive_element
    }

    fn photo_form(&mut self, ctx: &egui::Context) {
        let _photo_uuid = match &self.edit_photo_uuid {
            Some(uuid) => uuid.clone(),
            None => return,
        };

        egui::Window::new("View photo")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(500.)
                    .show(ui, |ui| {
                        let Some(texture) = &self.current_photo_image else { panic!("Original photo texture is missing") };
                        ui.add(sized_img_element(&ctx, texture, 540, 350));

                        for face_data in self.faces_form_data.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.add(sized_img_element(&ctx, &face_data.texture, 100, 100));

                                ui.label("Person:");

                                let mut selected_uuid = &face_data.selected_person_option.0;
                                let selected_text = &face_data.selected_person_option.1;
                                egui::ComboBox::new(&face_data.uuid, "")
                                    .selected_text(selected_text)
                                    .show_ui(ui, |ui| {
                                        for option in self.person_options.iter() {
                                                                // set value to,    value,     label 
                                            ui.selectable_value(&mut selected_uuid, &option.0, &option.1);
                                        }
                                    }
                                );

                                if !selected_uuid.is_empty() {
                                    let name = &self.person_options
                                                 .iter()
                                                 .find(|person| &person.0 == selected_uuid)
                                                 .unwrap().1;

                                    face_data.selected_person_option = (
                                        selected_uuid.clone(),
                                        name.clone()
                                        );
                                }
                            });
                        }
                    });

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.edit_photo_uuid = None;
                        self.current_photo_image = None;
                        self.person_options = Vec::new();
                        self.faces_form_data = Vec::new();
                    }

                    if ui.button("Save").clicked() {
                        for face_data in self.faces_form_data.iter() {
                            crate::db::face::Face::moderate_person(&face_data.uuid, &face_data.selected_person_option.0);
                        }

                        let ui_photo = self.photos
                            .iter_mut()
                             .find(|ui_photo| ui_photo.data.uuid == self.edit_photo_uuid.clone().unwrap())
                             .unwrap();

                        ui_photo.set_faces(ui_photo.data.faces());

                        self.edit_photo_uuid = None;
                        self.current_photo_image = None;
                        self.person_options = Vec::new();
                        self.faces_form_data = Vec::new();
                    }
                });

            });
    }
}

pub fn sized_img_element(
    ctx: &egui::Context,
    texture: &RetainedImage,
    width: u32,
    height: u32,
) -> egui::Image {
    let [original_width, original_height] = texture.size();
    let dimensions = crate::storage::resize_to_fit(
        &width,
        &height,
        &(original_width as u32),
        &(original_height as u32),
    );

    egui::Image::new(
        texture.texture_id(ctx),
        [dimensions.width as f32, dimensions.height as f32],
    )
}
