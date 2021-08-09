use diesel::prelude::*;
use myapp::models::models::Post;
use myapp::models::schema::posts as posts_schema;
use myapp::models::db_connect::establish_connection;

fn main() {
    let connection = establish_connection();
    //変更
    let posts = posts_schema::dsl::posts
        .load::<Post>(&connection)
        .expect("Error loading posts");
    for post in posts {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}