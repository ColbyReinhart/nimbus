// Nimbus webserver using rust rocket
// By Colby Reinhart
// 10-27-2022

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use std::fs::{read_dir};
use rocket_dyn_templates::Template;
use std::vec::Vec;

//
// Rocket boilerplate
//

#[launch]
fn rocket() -> _
{
	rocket::build()
		.mount("/", routes![
			homepage,
			get_static,
			get_cloud_resource
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

// Route user resources
#[get("/user-files/<user>/<filepath..>")]
async fn get_cloud_resource(user: &str, filepath: PathBuf) -> Template
{
	// If it's a directory, serve a list of files
	let resource_path: PathBuf = Path::new("user-files/").join(user).join(filepath);
	if resource_path.is_dir()
	{
		// Surely there's a better way to do this
		Template::render("file-explorer", context!{
			files: read_dir(&resource_path)
				.unwrap()
				.map(|entry| entry.unwrap().file_name().to_str().unwrap().to_owned())
				.map(|filename| String::from("<a href=\"/")
					+ resource_path.join(&filename).to_str().unwrap()
					+ "\">"
					+ &filename
					+ "<a>")
					.collect::<Vec<String>>()
		})
	}
	else
	{
		Template::render("view-file", context! [
			file: std::fs::read_to_string(resource_path).unwrap()
		])
	}
}