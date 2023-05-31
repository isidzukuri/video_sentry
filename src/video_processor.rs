extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::fs;
use image;
use uuid::Uuid;
use crate::storage;

// WIP
pub fn call(path_to_file: &String) -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    if let Ok(mut ictx) = input(path_to_file) {
        let tmp_folder = create_tmp_dir();

        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<bool, ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    let path_to_image = save_image(&rgb_frame, frame_index, &tmp_folder).unwrap();

                    if crate::image_processor::call(&path_to_image).is_some() { return Ok(true)};
                    frame_index += 1;
                }
                Ok(false)
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                match receive_and_process_decoded_frames(&mut decoder) {
                    Ok(true) => break,
                    Err(_) => todo!(),
                    Ok(false) => {},
                };
            }
        }
        decoder.send_eof()?;
        // receive_and_process_decoded_frames(&mut decoder)?;

        fs::remove_dir_all(tmp_folder).unwrap();
    }

    // TODO: return recognition result
    Ok(())
}

fn save_image(frame: &Video, index: usize, tmp_folder: &String) -> std::result::Result<String, std::io::Error> {
    let path_to_file = format!("{}/{}.jpg", tmp_folder, index);
    image::save_buffer_with_format(&path_to_file, 
                                   frame.data(0), 
                                   frame.width(), 
                                   frame.height(), 
                                   image::ColorType::Rgb8, 
                                   image::ImageFormat::Jpeg).unwrap();

    Ok(path_to_file)
}

fn create_tmp_dir() -> String {
    let folder_path = format!("{}/video/{}/", storage::TMP_DIR, Uuid::new_v4().to_string());
    match fs::create_dir_all(&folder_path) {
        Err(error) => panic!("Tmp folder cannot be created: {:?}", error),
        Ok(_) => folder_path,
    }
}
