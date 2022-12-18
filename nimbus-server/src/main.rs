// Nimbus webserver using rust rocket
// By Colby Reinhart
// 10-27-2022

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use std::path::{Path, PathBuf};
use std::fs::read_dir;
use std::vec::Vec;

use rocket::fs::{TempFile, NamedFile};
use rocket::form::Form;
use rocket::http::Status;
use rocket_dyn_templates::Template;

use nimbus_server::ResourceLink;

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
			put_cloud_resource
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
	// Get the resource path as a PathBuf
	let resource_path: PathBuf = Path::new("user-files/").join(user).join(filepath);
	
	// If it's a directory, we'll open it and display it's contents as a page
	if resource_path.is_dir()
	{
		Template::render("file-explorer", context! [
			// Read the contents of the directory and instantiate them as a
			// vector of resource links (see lib.rs)
			links: read_dir(&resource_path)
				.unwrap()
				.map(|entry| ResourceLink::from_dir_entry(entry.unwrap()))
				.collect::<Vec<ResourceLink>>()
		])
	}

	// Otherwise, we'll respond with the file
	else
	{
		Template::render("view-file", context! [
			file: std::fs::read_to_string(resource_path).unwrap()
		])
	}
}

// Structure for getting the uploaded file
#[derive(FromForm)]
pub struct FileUpload<'r>
{
	is_directory: bool,
	file: TempFile<'r>
}

impl<'r> FileUpload<'r> {
	// Move the file out of the struct
	pub fn take_file(self) -> TempFile<'r>
	{
		self.file
	}
}

// Upload user resource
#[post("/user-files/<filepath..>", data = "<form>")]
async fn put_cloud_resource(filepath: PathBuf, form: Form<FileUpload<'_>>) -> Status
{
	// Get the full path for the file based on the configured file root
	let full_path = Path::new("user-files/").join(filepath);

	// If it's a directory, create it
	if form.is_directory
	{
		match std::fs::create_dir(full_path)
		{
			Ok(()) => { return Status::Created },
			Err(_) => { return Status::InternalServerError }
		}
	}
	// Otherwise, create the file
	else
	{
		// Try to get the uploaded file and save it to full_path, return 500 otherwise
		match form.into_inner().take_file().persist_to(full_path).await
		{
			Ok(()) => {
				Status::Created
			},
			Err(what) => {
				print!("{}", what);
				Status::InternalServerError
			}
		}
	}
}