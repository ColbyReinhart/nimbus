extern crate serde;

use std::path::PathBuf;

use rocket::serde::Serialize;
use rocket::fs::TempFile;
use rocket::FromForm;

// Defines an HTML link tag to a server resource
#[derive(Serialize)]
pub struct ResourceLink
{
	pub path: PathBuf,
	pub name: String,
	pub is_dir: bool
}

// Structure for getting the uploaded file
#[derive(FromForm)]
pub struct FileUpload<'r>
{
	file: TempFile<'r>
}

impl<'r> FileUpload<'r>
{
	// Move the file out of the struct
	pub fn take_file(self) -> TempFile<'r>
	{
		self.file
	}
}