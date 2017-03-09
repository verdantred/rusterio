// Main server code

// New plugin feature in use to get macros included
#![feature(plugin)]
#![plugin(rocket_codegen, dotenv_macros)]

// Libs
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate serde_json;

// Libs with macros
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel_codegen;

// What to use
use rocket_contrib::JSON;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// My modules
pub mod models;
pub mod schema;

// Routes

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: JSON<models::User>) -> String{
    use schema::users;
    let con = connect_to_db();
    let n_user = user.into_inner();
    println!("{:?}", n_user);
    let rows = diesel::insert(&n_user).into(users::table).execute(&con).expect("Error saving new user");
    return format!("New user inserted, {:?} rows affected", rows);
}

#[get("/users")]
fn get_users() -> JSON<Vec<models::User>>{
    use schema::users::dsl::*;
    let con = connect_to_db();
    let ress = users.load::<models::User>(&con).expect("Error loading users");
    JSON(ress)
}

#[get("/user/<uid>")]
fn get_user(uid: i32) -> JSON<models::User>{
    use schema::users::dsl::*;
    let message = format!("Error finding user with id: {}", uid.to_string());
    let con = connect_to_db();
    let ress = users.find(uid).first(&con).expect(&message);
    JSON(ress)
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_user, get_users, new_user]).launch();
}

fn connect_to_db() -> SqliteConnection {
    let database_url = dotenv!("DATABASE_URL");
    SqliteConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url)) // return value
}