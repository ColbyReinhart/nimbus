// Nimbus webserver using rust rocket
// By Colby Reinhart
// 10-27-2022

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket_dyn_templates::Template;

//
// Rocket boilerplate
//

#[launch]
fn rocket() -> _
{
	rocket::build()
		.mount("/", routes!
		[
			homepage,
			get_static
		])
		.attach(Template::fairing())
}

// TODO: Implement favicon

// Route landing page
#[get("/")]
async fn homepage() -> Option<NamedFile>
{
	NamedFile::open(Path::new("templates/homescreen.html")).await.ok()
}

// Route static folder
#[get("/static/<file..>")]
async fn get_static(file: PathBuf) -> Option<NamedFile>
{
	NamedFile::open(Path::new("static/").join(file)).await.ok()	
}