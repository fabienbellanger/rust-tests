extern crate test_diesel;
extern crate diesel;

use self::test_diesel::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use test_diesel::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts\n", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
