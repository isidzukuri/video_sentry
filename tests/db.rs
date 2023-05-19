#[cfg(test)]
mod db_tests {
    use uuid::Uuid;
    use video_sentry::db;

    #[test]
    fn test_db_photo_crud() {
        db::init();

        // PHOTO
        let count_before = video_sentry::db::photo::Photo::all().len();
        let uuid = Uuid::new_v4().to_string();
        let item = video_sentry::db::photo::Photo::create(&uuid);
        let count_after = video_sentry::db::photo::Photo::all().len();

        assert_eq!(item.uuid, uuid);
        assert_eq!((count_after - count_before), 1);

        let item = video_sentry::db::photo::Photo::find(&uuid);
        assert_eq!(item.uuid, uuid);

        let count = video_sentry::db::photo::Photo::count();
        assert_eq!((count > 0), true);

        video_sentry::db::photo::Photo::delete(&uuid);

        let uuid = Uuid::new_v4().to_string();
        video_sentry::db::photo::Photo::create(&uuid);
        let count_before = video_sentry::db::photo::Photo::all().len();

        video_sentry::db::photo::Photo::delete(&uuid);

        let count_after = video_sentry::db::photo::Photo::all().len();

        assert_eq!((count_before - count_after), 1);

        let uuid_1 = Uuid::new_v4().to_string();
        video_sentry::db::photo::Photo::create(&uuid_1);

        let uuid_2 = Uuid::new_v4().to_string();
        video_sentry::db::photo::Photo::create(&uuid_2);

        let items = video_sentry::db::photo::Photo::where_all(&format!(
            "uuid IN('{uuid_1}', '{uuid_2}')",
            uuid_1 = uuid_1,
            uuid_2 = uuid_2
        ));

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].uuid, uuid_1);
        assert_eq!(items[1].uuid, uuid_2);
    }
}
