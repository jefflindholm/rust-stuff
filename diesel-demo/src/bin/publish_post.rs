extern crate diesel;
extern crate simple_api;

use self::diesel::prelude::*;
use self::models::Post;
use self::simple_api::*;
use std::env::args;

fn main() {
    use simple_api::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i64>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
