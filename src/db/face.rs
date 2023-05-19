use crate::db::*;
use debug_print::debug_println;
use serde_json::json;
use sqlx::FromRow;
use colored::Colorize;

#[derive(Clone, FromRow, Debug)]
pub struct Face {
    pub id: i64,
    pub uuid: String,
    pub photo_uuid: String,
    pub person_uuid: String,
    pub measurements: String,
}

impl Face {
    fn table_name() -> String {
        String::from("faces")
    }

    pub fn serialize_measurements(data: &Vec<f64>) -> String {
        json!(data).to_string()
    }

    pub fn deserialize_measurements(&self) -> Vec<f64> {
        match serde_json::from_str::<Vec<f64>>(&self.measurements.to_string()) {
            Err(error) => panic!("deserialize_measurements failed: {}", error),
            Ok(json) => json,
        }
    }

    #[tokio::main]
    pub async fn create(uuid: &String, photo_uuid: &String, measurements: &String) -> Self {
        let result = sqlx::query(&format!(
            "INSERT INTO {table_name} (uuid, photo_uuid, measurements) VALUES (?, ?, ?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .bind(photo_uuid)
        .bind(measurements)
        .execute(connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                Self {
                    id: data.last_insert_rowid(),
                    uuid: uuid.clone(),
                    photo_uuid: photo_uuid.clone(),
                    person_uuid: "".to_string(),
                    measurements: measurements.clone(),
                }
            }
        }
    }

    #[tokio::main]
    pub async fn update_person_uuid(uuid: &String, person_uuid: &String) -> bool {
        let result = sqlx::query(&format!(
            "UPDATE {table_name} SET person_uuid = (?) WHERE uuid = (?)",
            table_name = &Self::table_name()
        ))
        .bind(person_uuid)
        .bind(uuid)
        .execute(connection())
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
    pub async fn update(uuid: &String, params: Vec<(&String, &String)>) -> bool {
        let mut statement = String::from("");
        for (i, param) in params.iter().enumerate() {
            let separator = if i < 1 { "" } else { ", " };
            statement.push_str(separator);
            statement.push_str(
                format!("{column} = '{value}'", column = param.0, value = param.1).as_str(),
            );
        }

        let query_str = &format!(
            "UPDATE {table_name} SET {} WHERE uuid = (?)",
            statement,
            table_name = &Self::table_name()
        );

        debug_println!("{}", query_str.cyan());

        let result = sqlx::query(query_str)
            .bind(uuid)
            .execute(connection())
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
    pub async fn find(uuid: &String) -> Self {
        let result = sqlx::query_as::<_, Self>(&format!(
            "SELECT * FROM {table_name} WHERE uuid = (?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .fetch_one(connection())
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
            "SELECT * FROM {table_name}",
            table_name = &Self::table_name()
        ))
        .fetch_all(connection())
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
            .fetch_all(connection())
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
        .execute(connection())
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
        .fetch_one(connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => data.total_count,
        }
    }
}
