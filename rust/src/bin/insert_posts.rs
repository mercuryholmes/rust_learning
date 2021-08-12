use anyhow::{Result};
use diesel::prelude::*;
use myapp::models::models::NewPost;
use myapp::models::schema::posts as posts_schema;
use myapp::models::db_connect::establish_connection;

fn main() -> Result<()> {
    let connection = establish_connection();
    let new_posts = vec![
        NewPost {
            title: String::from("2021/08/01"),
            body: String::from("今日は何もしなかった。"),
        },
        NewPost {
            title: String::from("2021/08/02"),
            body: String::from("今日はプログラミングをした。"),
        },
    ];
    diesel::insert_into(posts_schema::dsl::posts)
        .values(&new_posts)
        .execute(&connection)
        .expect("Error insert new post");
    Ok(())
}