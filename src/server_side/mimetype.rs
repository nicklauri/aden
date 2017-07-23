
use std::path;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

use server_side::utils;

#[derive(Debug)]
pub struct Mimetype {
	mimetype_vec: Vec<MimetypeData>, 
	pub default_mimetype: String
}

#[derive(Debug)]
pub struct MimetypeData {
	extension: String,
	mimetype: String
}

impl Mimetype {
	pub fn new() -> Result<Mimetype, io::Error> {
		let mut mimetypes: Vec<MimetypeData> = vec![];
		let root_path = utils::get_root_path();


		// Process custom mimetype first!
		// For safety, Use std::path::PathBuf to create path to compatible to cross platform.
		let custom_mimetype_pathbuf = path::PathBuf::from(utils::to_root_path("/config/custom_mimetype.mt", &root_path));
		let custom_mimetype_path: &str = custom_mimetype_pathbuf.to_str().unwrap();
		let custom_mimetype_file_handle = match File::open(custom_mimetype_path)  {
			Ok(ok) => ok,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("can't find `{:?}`", custom_mimetype_path)))
		};
		let custom_mimetype_bufreader = BufReader::new(&custom_mimetype_file_handle);

		for line_wrapped in custom_mimetype_bufreader.lines() {
			let line_raw = line_wrapped.unwrap();
			// line_raw.trim();
			// line_raw.trim_matches('\r');
			// line_raw.trim_matches('\n');
			// line_raw.trim_matches('\t');

			let line = line_raw.trim();
			if line.len() > 0 && line.as_bytes()[0] != '#' as u8 {
				let line_splitted = line.split('\t').collect::<Vec<&str>>();
				let key = line_splitted[0];
				let val = line_splitted[1];
				mimetypes.push(MimetypeData::new(key.to_string(), val.to_string()));
			}
		}

		// Then, server mimetype
		let mimetype_pathbuf = path::PathBuf::from(utils::to_root_path("/config/mimetype.mt", &root_path));
		let mimetype_path: &str = mimetype_pathbuf.to_str().unwrap();
		let mimetype_file_handle = match File::open(mimetype_path) {
			Ok(ok) => ok,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("can't find `{:?}`", mimetype_path)))
		};
		let mimetype_bufreader = BufReader::new(&mimetype_file_handle);

		for line_wrapped in mimetype_bufreader.lines() {
			let line_raw = line_wrapped.unwrap();
			// line_raw.trim();
			// line_raw.trim_matches('\r');
			// line_raw.trim_matches('\n');
			// line_raw.trim_matches('\t');

			let line = line_raw.trim();
			if line.len() > 0 && line.as_bytes()[0] != '#' as u8 {
				let line_splitted = line.split('\t').collect::<Vec<&str>>();
				let key = line_splitted[0];
				let val = line_splitted[1];
				mimetypes.push(MimetypeData::new(key.to_string(), val.to_string()));
			}
		}

		Ok(Mimetype{ mimetype_vec: mimetypes, default_mimetype: "text/plain".to_string() })
	}

	pub fn get_mimetype(&self, filename: &String) -> Result<String, io::Error> {
		let ext: String;
		if filename.contains(".") {
			let filename_ext = filename.split(".").last().unwrap().to_string();
			ext = ".".to_string() + filename_ext.as_str();

			for mt in self.mimetype_vec.iter() {
				if mt.get_extension() == ext.as_str() {
					return Ok(mt.get_mimetype());
				}
			}
		}

		Err(io::Error::new(io::ErrorKind::Other, "mimetype not found."))
	}

	pub fn get_mimetype_or(&self, filename: &String, default: &str) -> String {
		match self.get_mimetype(filename) {
			Ok(mt) => mt,
			Err(_) => default.to_owned()
		}
	}

	pub fn get_mimetype_default(&self, filename: &String) -> String {
		match self.get_mimetype(filename) {
			Ok(mt) => mt,

			// TODO: 
			Err(_) => self.default_mimetype.to_owned(),
		}
	}

	pub fn to_owned(&self) -> Mimetype {
		let mut new_mt: Vec<MimetypeData> = vec![];
		for mt in self.mimetype_vec.iter() {
			new_mt.push(MimetypeData{extension: mt.get_extension(), mimetype: mt.get_mimetype()})
		}

		Mimetype {
			mimetype_vec: new_mt,
			default_mimetype: self.default_mimetype.to_owned()
		}
	}

	pub fn delete(self) {}
}

impl MimetypeData {
	pub fn new(e: String, m: String) -> MimetypeData {
		MimetypeData{extension: e, mimetype: m}
	}

	pub fn get_extension(&self) -> String {
		self.extension.to_owned()
	}

	pub fn get_mimetype(&self) -> String {
		self.mimetype.to_owned()
	}

	pub fn set_mimetype(&mut self, e: String) {
		self.mimetype = e;
	}

	pub fn delete(self) {}
}

