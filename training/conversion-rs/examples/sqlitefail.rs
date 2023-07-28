use sqlx::{sqlite::SqlitePoolOptions, SqliteConnection, Connection, Row};

use sqlx::{SqlitePool};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // need the ?mode
    let mut new_db = SqliteConnection::connect("sqlite://test.db?mode=rwc").await?;
    
    // let pool = SqlitePoolOptions::new()
    //     .create_if_missing()
    // ::connect("sqlite://test.db")
    //     .await
    //     .expect("Could not connect to sqlite db!!!");
    Ok(())
}