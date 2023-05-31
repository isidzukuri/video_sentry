mod common;

use video_sentry::db::photo::Photo;

#[test]
fn directory_trainer() {
    common::setup();
    common::pretrain();

    let photos = Photo::all();

    assert_eq!(photos[0].faces()[0].person().unwrap().name, "marion");
    assert_eq!(photos[1].faces()[0].person().unwrap().name, "armas");
    assert_eq!(photos[2].faces()[0].person().unwrap().name, "armas");

    common::cleanup();
}