use crate::db;
use crate::db::*;
use colored::Colorize;
use debug_print::debug_println;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Photo {
    pub id: i64,
    pub uuid: String,
}

impl Photo {
    fn table_name() -> String {
        String::from("photos")
    }

    pub fn save_detection_result(photo: &crate::detection::photo::Photo) {
        crate::db::photo::Photo::create(&photo.uuid);
        for face in photo.faces.iter() {
            crate::db::face::Face::create(
                &face.uuid,
                &photo.uuid,
                &crate::db::face::Face::serialize_measurements(&face.measurements),
            );
        }
    }

    #[tokio::main]
    pub async fn create(uuid: &String) -> Self {
        let result = sqlx::query(&format!(
            "INSERT INTO {table_name} (uuid) VALUES (?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .execute(db::connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                Self {
                    id: data.last_insert_rowid(),
                    uuid: uuid.clone(),
                }
            }
        }
    }

    #[tokio::main]
    pub async fn find(uuid: &String) -> Self {
        let result = sqlx::query_as::<_, Self>(&format!(
            "SELECT * FROM {table_name} WHERE uuid = (?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .fetch_one(db::connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                data
            }
        }
    }

    #[tokio::main]
    pub async fn all() -> Vec<Self> {
        let result = sqlx::query_as::<_, Self>(&format!(
            "SELECT * FROM {table_name} ORDER BY id DESC",
            table_name = &Self::table_name()
        ))
        .fetch_all(db::connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                data
            }
        }
    }

    #[tokio::main]
    pub async fn where_all(statement: &String) -> Vec<Self> {
        let query_str = &format!(
            "SELECT * FROM {table_name} WHERE {};",
            statement,
            table_name = &Self::table_name()
        );

        debug_println!("{}", query_str.cyan());

        let result = sqlx::query_as::<_, Self>(query_str)
            .fetch_all(db::connection())
            .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                data
            }
        }
    }

    #[tokio::main]
    pub async fn delete(uuid: &String) -> bool {
        let result = sqlx::query(&format!(
            "DELETE FROM {table_name} WHERE uuid = (?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .execute(db::connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                true
            }
        }
    }

    #[tokio::main]
    pub async fn count() -> i64 {
        let result = sqlx::query_as::<_, Collection>(&format!(
            "SELECT COUNT(*) as total_count FROM {table_name}",
            table_name = &Self::table_name()
        ))
        .fetch_one(db::connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => data.total_count,
        }
    }

    pub fn faces(&self) -> Vec<crate::db::face::Face> {
        crate::db::face::Face::where_all(&format!("photo_uuid = '{}'", self.uuid))
    }
}
