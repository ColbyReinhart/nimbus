// Nimbus webserver using rust rocket
// By Colby Reinhart
// 10-27-2022

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::fs::NamedFile;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;
use std::path::{Path, PathBuf};
use std::fs::{Metadata, metadata, read_dir};

//
// Rocket boilerplate
//

#[launch]
fn rocket() -> _
{
	rocket::build().mount("/", routes![
		homepage,
		get_static,
		get_cloud_directory,
		get_cloud_file
	])
}

//
// Create request guard to differentiate between file and directory retrieval
//

pub struct ResourceType;

#[derive(Debug)]
pub enum ResourceTypeError
{
	InvalidResource
}

#[rocket::async_trait]
impl <'r> FromRequest<'r> for ResourceType
{
	type Error = ResourceTypeError;

	async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
	{
		let data: Metadata = metadata(req.route().unwrap().to_string())
			.expect("Couldn't get file metadata");
		
		if data.is_dir()
		{
			return Outcome::Success(Self);
		}
		else if data.is_file()
		{
			return Outcome::Forward(());
		}
		else
		{
			return Outcome::Failure((Status::Forbidden, ResourceTypeError::InvalidResource));	
		}
	}
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

// Route user files
// The first handler treats it as a directory, either forwarding it to the
// next handler if it's a file or throwing and error if it's neither
// TODO: implement security here
#[get("/user-files/<user>/<filepath..>")]
async fn get_cloud_directory(user: &str, filepath: PathBuf, _resource: ResourceType)
	-> String
{
	// Anything that made it past this include guard is a directory
	let path: PathBuf = Path::new("/user-files/").join(user).join(filepath);
	let mut res: String = "<!DOCTYPE html><html><body>".to_string();

	for file in read_dir(path).expect("Could not open directory")
	{
		let filename: String = file.unwrap().path().display().to_string();
		res.push_str("<a href=\"");
		res.push_str(&filename);
		res.push_str("\">");
		res.push_str(&filename);
		res.push_str("</a><br>");
	}

	res.push_str("</body></html>");
	return res;
}

async fn get_cloud_file(user: &str, filepath: PathBuf)
	-> Option<NamedFile>
{
	NamedFile::open(Path::new("/user-files/").join(user).join(filepath)).await.ok()
}