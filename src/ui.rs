use std::fs::File;
use std::sync::mpsc;
use std::thread;

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
}

impl VsUi {
    fn new(_context: &eframe::CreationContext<'_>) -> Self {
        let (mut photos_tx, photos_rx) = mpsc::channel();
        let instance = Self {
            photos: Vec::new(),
            people: crate::db::person::Person::all(),
            photos_rx: Some(photos_rx),
        };

        thread::spawn(move || {
            Self::fetch_photos(&mut photos_tx);
        });

        instance
    }

    fn fetch_photos(
        photos_tx: &mut std::sync::mpsc::Sender<UIPhoto>
    ) {
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

    pub fn preload_photos(&mut self) {
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

    pub fn render_photos(&self, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
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
}

impl Default for VsUi {
    fn default() -> Self {
        Self {
            photos: Vec::new(),
            people: Vec::new(),
            photos_rx: None,
        }
    }
}

impl eframe::App for VsUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.preload_photos();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_photos(ui, ctx);
            });
        });
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
