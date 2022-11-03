extern crate serde;

use std::path::PathBuf;

use rocket::serde::Serialize;

// Defines an HTML link tag to a server resource
#[derive(Serialize)]
pub struct ResourceLink
{
	pub path: PathBuf,
	pub name: String
}