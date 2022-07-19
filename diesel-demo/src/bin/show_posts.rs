extern crate diesel;
extern crate simple_api;

use self::models::*;
use diesel::prelude::*;
use simple_api::*;

fn main() {
    use simple_api::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.author);
        println!("----------\n");
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
