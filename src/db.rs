use debug_print::debug_println;
use dotenvy::dotenv;
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Sqlite, SqlitePool};
use std::env;
use std::fs;
use std::sync::Once;

pub mod face;
pub mod person;
pub mod photo;

static mut CONNECTION: Option<Pool<Sqlite>> = None;
static INIT: Once = Once::new();

pub fn init() {
    unsafe {
        INIT.call_once(|| {
            CONNECTION = Some(prepare());
        });
    }
}

pub fn connection() -> &'static Pool<Sqlite> {
    unsafe {
        match &CONNECTION {
            Some(pool) => &pool,
            None => panic!("Cant obtain connection pool"),
        }
    }
}

#[tokio::main]
pub async fn prepare() -> Pool<Sqlite> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        let database_folder = env::var("DATABASE_FOLDER").expect("DATABASE_FOLDER must be set");
        match fs::create_dir_all(database_folder) {
            Err(error) => panic!("Database folder cannot be created: {:?}", error),
            Ok(_) => {
                debug_println!("Database folder is created")
            }
        }

        debug_println!("Creating database {}", &database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => {
                debug_println!("Create db success")
            }
            Err(error) => panic!("error: {}", error),
        }
    } else {
        debug_println!("Database already exists");
    }

    let db = SqlitePool::connect(&database_url).await.unwrap();

    let migrations_dir = std::env::var("MIGRATIONS_DIR").unwrap();
    let migrations = std::path::Path::new(&migrations_dir);
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => {
            debug_println!("Migration success")
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    debug_println!("migration: {:?}", migration_results);

    db
}

#[derive(Clone, FromRow, Debug)]
pub struct Collection {
    pub total_count: i64,
}

// https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
