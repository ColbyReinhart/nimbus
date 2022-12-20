// Nimbus webserver using rust rocket
// By Colby Reinhart
// 10-27-2022

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use std::path::{Path, PathBuf};
use std::fs::read_dir;
use std::vec::Vec;

use rocket::fs::{NamedFile};
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
			get_file,
			get_directory
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
#[get("/static/<file..>", rank = 1)]
async fn get_static(file: PathBuf) -> Option<NamedFile>
{
	NamedFile::open(Path::new("static/").join(file)).await.ok()	
}

// Route user file
#[get("/<user>/file/<filepath..>", rank = 2)]
async fn get_file(user: &str, filepath: PathBuf) -> Result<Option<NamedFile>, Status>
{
	// Construct actual filepath
	let full_path: PathBuf = Path::new("user-files/").join(&user).join(filepath);

	// Return 404 if it's a directory
	if full_path.is_dir()
	{
		Err(Status::NotFound)
	}

	// Otherwise, return the requested file (if it exists)
	else
	{
		Ok(NamedFile::open(full_path).await.ok())
	}
}

// Route user directory
#[get("/<user>/folder/<filepath..>")]
async fn get_directory(user: &str, filepath: PathBuf) -> Result<Template, Status>
{
	// Construct actual filepath
	let full_path: PathBuf = Path::new("user-files/")
		.join(&user)
		.join(&filepath);

	// If it's not a directory, return a 404
	if !full_path.is_dir()
	{
		return Err(Status::NotFound)
	}

	// Otherwise, fill and return the directory template
	let mut files: Vec<ResourceLink> = Vec::new();
	for entry in read_dir(&full_path).unwrap().into_iter()
	{
		// Get the name of the resource
		let file_name: String = entry.unwrap()
			.file_name()
			.to_str()
			.unwrap()
			.to_owned();

		// Get the local path to the resource
		let real_path: PathBuf = full_path.join(&file_name);

		// Now get the URL path
		let url: PathBuf = PathBuf::new().join(user)
			.join(if real_path.is_dir() {"folder"} else {"file"})
			.join(&filepath)
			.join(&file_name);

		// Add it to the list
		files.push
		(
			ResourceLink
			{
				path: url,
				name: file_name,
				is_dir: real_path.is_dir()
				
			}
		);
	}

	Ok(Template::render("directory", context![links: files]))
}

// // Upload user file
// #[post("/user-files/<filepath..>", data = "<form>")]
// async fn put_cloud_file(filepath: PathBuf, form: Form<nimbus_server::FileUpload<'_>>) -> Status
// {
// 	// Get the full path for the file based on the configured file root
// 	let full_path: PathBuf = Path::new("user-files/").join(filepath);

// 	// Try to get the uploaded file and save it to full_path, return 500 otherwise
// 	match form.into_inner().take_file().persist_to(full_path).await
// 	{
// 		Ok(()) => {
// 			Status::Created
// 		},
// 		Err(what) => {
// 			print!("{}", what);
// 			Status::InternalServerError
// 		}
// 	}
// }

// if form.is_directory
// 	{
// 		match std::fs::create_dir(full_path)
// 		{
// 			Ok(()) => { return Status::Created },
// 			Err(_) => { return Status::InternalServerError }
// 		}
// 	}