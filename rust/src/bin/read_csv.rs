use anyhow::{Result};
use diesel::prelude::*;
use csv::ReaderBuilder;
use myapp::models::models::NewPost;
use myapp::models::schema::posts as posts_schema;
use myapp::models::db_connect::establish_connection;

fn main() -> Result<()> {
    // DBに接続
    let connection = establish_connection();
    // CSV読み込み
    let mut rdr = ReaderBuilder::new()
    .has_headers(false)
    .from_path("./foo.csv")?;
    for result in rdr.records() {
        let record = result?;
        let id = &record[0];
        let title = &record[1];
        let body = &record[2];
        println!("{}, {1}, {2}", &id, &title, &body);
        // DBに格納
        let new_post = NewPost {
            title: title.to_string(),
            body: body.to_string(),
        };
        diesel::insert_into(posts_schema::dsl::posts)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");
    }
    Ok(())
}