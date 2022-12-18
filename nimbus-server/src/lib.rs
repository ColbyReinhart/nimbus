extern crate serde;

use std::path::PathBuf;
use std::fs::DirEntry;

use rocket::serde::Serialize;
use rocket::fs::TempFile;
use rocket::FromForm;

// Defines an HTML link tag to a server resource
#[derive(Serialize)]
pub struct ResourceLink
{
	pub path: PathBuf,
	pub name: String
}

impl ResourceLink
{
	pub fn from_dir_entry(entry: DirEntry) -> Self
	{
		ResourceLink
		{
			path: entry.path().to_path_buf(),
			name: entry.file_name().to_str().unwrap().to_owned()
		}
	}
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