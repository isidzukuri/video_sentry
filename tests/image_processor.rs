mod common;

use video_sentry::db::photo::Photo;
use video_sentry::db::person::Person;
use video_sentry::db::face::Face;

#[test]
fn image_processor() {
    common::setup();
    common::pretrain();

    let people_count_before = Person::all().len();
    let photo_count_before = Photo::all().len();
    let face_count_before = Face::all().len();
    let result = video_sentry::image_processor::call(&"tests/fixtures/people/marion/2.jpg".to_string()).unwrap();
    let people_count_diff = Person::all().len() - people_count_before;
    let photo_count_diff = Photo::all().len() - photo_count_before;
    let face_count_diff = Face::all().len() - face_count_before;

    assert_eq!(result.display_data[0].0, "marion");
    assert_eq!(people_count_diff, 0);
    assert_eq!(photo_count_diff, 1);
    assert_eq!(face_count_diff, 1);
    assert_eq!(result.is_face_found(), true);

    let people_count_before = Person::all().len();
    let photo_count_before = Photo::all().len();
    let face_count_before = Face::all().len();
    let result = video_sentry::image_processor::call(&"tests/fixtures/people/deniro/1.jpg".to_string()).unwrap();
    let people_count_diff = Person::all().len() - people_count_before;
    let photo_count_diff = Photo::all().len() - photo_count_before;
    let face_count_diff = Face::all().len() - face_count_before;

    assert_eq!(result.display_data.len(), 0);
    assert_eq!(people_count_diff, 0);
    assert_eq!(photo_count_diff, 1);
    assert_eq!(face_count_diff, 1);
    assert_eq!(result.is_face_found(), true);

    let result = video_sentry::image_processor::call(&"tests/fixtures/1px.jpg".to_string());
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().is_face_found(), false);

    common::cleanup();
}