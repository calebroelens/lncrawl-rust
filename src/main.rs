use std::collections::HashMap;
use std::fs::File;
use rocket::{Rocket, routes, State};
use rocket::fs::NamedFile;
use rocket::get;
use rocket::http::Status;
use rocket_dyn_templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<&str,bool> = HashMap::new();
    return Template::render("index", &context);
}

#[get("/ping")]
pub fn ping() -> String{
    return String::from("pong");
}

#[get("/resource/<directory>/<filename>")]
pub async fn resource(directory: &str, filename: &str) -> (Status, Option<NamedFile>) {
    let mut script = NamedFile::open(format!("{}/{}", directory, filename)).await;
    return if script.is_err() {
        // Failed to get the file.
        (Status::new(404), None)
    } else {
        (Status::new(200), Some(script.unwrap()))
    }
}

#[rocket::main]
async fn main(){
    println!("Starting webserver application");

    let rocket = rocket::build()
        .mount("/", routes!(ping, index, resource))
        .attach(Template::fairing());

    let launch_result = rocket.launch().await;
    println!("Server closed or crashed!: {:?}", launch_result);
}
