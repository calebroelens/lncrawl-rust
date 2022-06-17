mod source;
mod novel;

use std::collections::HashMap;
use std::fs::File;
use rocket::{Rocket, routes, State};
use rocket::fs::NamedFile;
use rocket::{get, post};
use rocket::http::Status;
use rocket_dyn_templates::Template;
use novel::novel_fetch;

#[get("/")]
pub fn index() -> Template {
    let mut context: HashMap<&str,&str> = HashMap::new();
    context.insert("page", "home");
    return Template::render("index", &context);
}

#[get("/find")]
pub fn find() -> Template {
    let mut context: HashMap<&str,&str> = HashMap::new();
    context.insert("page", "find");
    return Template::render("find", &context);
}

#[get("/info")]
pub fn info() -> Template {
    let mut context: HashMap<&str,&str> = HashMap::new();
    context.insert("page", "info");
    return Template::render("info", &context);
}

#[get("/error")]
pub fn error() -> Template {
    let mut context: HashMap<&str,&str> = HashMap::new();
    return Template::render("error", &context);
}

#[get("/sources")]
pub fn sources() -> Template {
    let mut context: HashMap<&str,&str> = HashMap::new();
    context.insert("page", "info");
    return Template::render("sources", &context);
}

#[get("/ping")]
pub fn ping() -> String{
    return String::from("pong");
}

#[post("/novel_info", format="json", data="<data>")]
pub fn novel_info(data: rocket::serde::json::Json<novel_fetch::NovelInfo>) -> Template {
    unimplemented!();
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
        .mount("/", routes!(ping, index, resource, error, info, find, novel_info))
        .attach(Template::fairing());

    let launch_result = rocket.launch().await;
    println!("Server closed or crashed!: {:?}", launch_result);
}
