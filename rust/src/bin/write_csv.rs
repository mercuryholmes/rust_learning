use std::error;
use csv::Writer;
use diesel::prelude::*;
use myapp::models::models::Post;
use myapp::models::schema::posts as posts_schema;
use myapp::models::db_connect::establish_connection;

fn main() -> Result<(), Box<dyn error::Error>> {
    // データの取得
    let connection = establish_connection();
    let posts = posts_schema::dsl::posts
        .load::<Post>(&connection)
        .expect("Error loading posts");
    // CSVに書込み
    let mut wtr = Writer::from_path("foo.csv")?;
    for post in posts {
        println!("{}", post.title);
        println!("{}", post.body);
        wtr.write_record(&[post.title, post.body])?;
    }
    wtr.flush()?;
    Ok(())
}