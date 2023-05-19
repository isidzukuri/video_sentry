use crate::db::*;
use debug_print::debug_println;
use sqlx::FromRow;
use colored::Colorize;

#[derive(Clone, FromRow, Debug, PartialEq)]
pub struct Person {
    pub id: i64,
    pub uuid: String,
    pub name: String,
}

impl Person {
    fn table_name() -> String {
        String::from("persons")
    }

    #[tokio::main]
    pub async fn create(uuid: &String, name: &String) -> Self {
        let result = sqlx::query(&format!(
            "INSERT INTO {table_name} (uuid, name) VALUES (?, ?)",
            table_name = &Self::table_name()
        ))
        .bind(uuid)
        .bind(name)
        .execute(connection())
        .await;

        match result {
            Err(error) => panic!("error: {}", error),
            Ok(data) => {
                debug_println!("Query result: {:?}", data);

                Self {
                    id: data.last_insert_rowid(),
                    uuid: uuid.clone(),
                    name: name.clone(),
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
