use sqlx::{PgPool,postgres::PgPoolOptions};

pub async fn create_db_pool(max_conn: u32) -> PgPool {
    // getting db url from env
    let db_url = std::env::var("DATABASE_URL")
        .expect("environment variable DATABASE_URL not present");

    // creating connection
   let db_pool =  PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&db_url)
       .await
       .expect("failed to create connection to database");

    println!("connection to database success");

    // migration check
    sqlx::migrate!("./src/migrations")
        .run(&db_pool)
        .await
        .expect("failed to run or check database migration");
    println!("database migration executed");

    db_pool
}