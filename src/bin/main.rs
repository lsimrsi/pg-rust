extern crate diesel;
use pg_rust;
use pg_rust::*;

use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.first);
        println!("-----------\n");
        println!("{}", user.last);
    }
}