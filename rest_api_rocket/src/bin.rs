#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use lib::model;
use lib::db;
use lib::model::Movie;

use lib::routes::{index, create};


// #[get("/movies")]
// fn get_movies() -> Json<Option<Vec<Movie>>> {
//     Json(db::read_movies())
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api/v1", routes![index, create])
    .mount("/api/user", routes![create])
}

// fn main() {
//     println!("Hello, world!");
// }
