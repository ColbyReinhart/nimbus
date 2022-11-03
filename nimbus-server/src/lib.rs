extern crate serde;

use std::path::PathBuf;
use std::fs::DirEntry;

use rocket::serde::Serialize;

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