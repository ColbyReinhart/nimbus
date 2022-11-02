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
			get_cloud_resource,
			template
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
async fn get_cloud_resource(user: &str, filepath: PathBuf) -> Result<String, std::io::Error>
{
	// If it's a directory, serve a list of files
	let resource_path: PathBuf = Path::new("user-files/").join(user).join(filepath);
	if resource_path.is_dir()
	{
		let mut res: String = "<!DOCTYPE html><html><body>".to_string();

		for file in read_dir(resource_path).expect("Could not open directory")
		{
			let filename: String = file.unwrap().path().display().to_string();
			res.push_str("<a href=\"");
			res.push_str(&filename);
			res.push_str("\">");
			res.push_str(&filename);
			res.push_str("</a><br>");
		}

		res.push_str("</body></html>");
		Ok(res)
	}
	else
	{
		std::fs::read_to_string(resource_path)
	}
}

// Testing templates
#[get("/template")]
fn template() -> Template
{
	let vector = Vec::from([1, 2, 3, 4, 5]);
	Template::render("index", context! {
		numbers: vector
	})
}