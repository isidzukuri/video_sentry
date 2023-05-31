mod common;

use video_sentry::db::photo::Photo;
use video_sentry::db::person::Person;
use video_sentry::db::face::Face;

#[test]
fn video_processor() {
    common::setup();
    common::pretrain();

    let people_count_before = Person::all().len();
    let photo_count_before = Photo::all().len();
    let face_count_before = Face::all().len();
    let _result = video_sentry::video_processor::call(&"tests/fixtures/video/1.mp4".to_string()).unwrap();
    let people_count_diff = Person::all().len() - people_count_before;
    let photo_count_diff = Photo::all().len() - photo_count_before;
    let face_count_diff = Face::all().len() - face_count_before;

    // assert_eq!(result.display_data[0].0, "video_woman");
    assert_eq!(Face::all().last().unwrap().person().unwrap().name, "video_woman");
    assert_eq!(people_count_diff, 0);
    assert_eq!(photo_count_diff, 1);
    assert_eq!(face_count_diff, 1);

    common::cleanup();
}