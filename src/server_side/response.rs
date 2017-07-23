/**
 *
 */

use std::fs::{File, Metadata, metadata};
use std::io::{Read, BufReader};
use std::io::{Error, ErrorKind};

pub struct Response {
	http_ver: String,
	pub status_code: u16,
	status_msg: String,
	header: Vec<HeaderData>,
	content: Vec<u8>,
	content_length: usize,
	is_ready: bool
}

pub struct HeaderData {
	pub key: String,
	pub value: String
}

impl Response {
	pub fn new() -> Response {
		Response {
			http_ver: "".to_string(),
			status_code: 0,
			status_msg: "".to_string(),
			header: vec![],
			content: vec![],
			content_length: 0,
			is_ready: false
		}
	}

	pub fn add_header(&mut self, k: String, v: String) {
		// Don't allow to change content-length
		if k.to_lowercase() == "content-length" {
			return;
		}

		// No duplicate, just override
		// If match, move it and return.
		for hd in self.header.iter_mut() {
			if hd.key.to_lowercase() == k.to_lowercase() {
				hd.value = v;
				return;
			}
		}

		self.header.push(HeaderData{ key: k, value: v });
	}

	// Safe method, delete if exist and do nothing if not found.
	pub fn remove_header(&mut self, k: String) {
		if let Some(index) = self.header.iter()
			.position(|x| *x.key.to_lowercase() == k.to_lowercase()) {
			self.header.remove(index);
		}
	}

	pub fn set_response_text(&mut self, h: Option<String>, c: Option<u16>, m: Option<String>) {
		if let Some(http_ver) = h {
			self.http_ver = http_ver;
		}

		if let Some(status_code) = c {
			self.status_code = status_code;
		}

		if let Some(status_msg) = m {
			self.status_msg = status_msg;
		}
	}

	pub fn add_content(&mut self, filepath: String) -> Result<(), Error> {
		self.add_content_from_file(filepath)
	}

	// Enter file path!
	pub fn add_content_from_string(&mut self, content: String) {
		if !content.is_empty() {
			self.content_length = content.len();
			self.content = content.as_bytes().to_vec();
		}
	}

	pub fn add_content_from_file(&mut self, filepath: String) -> Result<(), Error> {
		let fp: File = File::open(filepath.to_owned())?;
		let mut fp_bufreader = BufReader::new(&fp);
		let file_size = metadata(filepath).unwrap().len();
		let mut content: Vec<u8> = vec![0; file_size as usize];
		let content_length: usize = fp_bufreader.read(&mut content)?;

		self.content = content;
		self.content_length = content_length;
		Ok(())
	}

	pub fn check_ready(&mut self) -> bool {
		if !self.is_ready {
			if !self.http_ver.is_empty() && self.status_code != 0
				&& !self.status_msg.is_empty() && self.header.len() > 0
				&& self.content_length > 0 {
					self.is_ready = true;
			}
		}

		// println!("self.http_ver.is_empty(): {:?}", self.http_ver.is_empty());
		// println!("self.status_code: {}", self.status_code);
		// println!("self.status_msg.is_empty(): {:?}", self.status_msg.is_empty());
		// println!("self.header.len(): {}", self.header.len());
		// println!("self.content_length: {}", self.content_length);

		self.is_ready
	}

	pub fn build_header(&mut self) -> Result<String, Error> {
		if !self.check_ready() {
			return Err(Error::new(ErrorKind::Other, "response is incomplete."));
		}

		let mut res: String;
		res = format!("HTTP/{} {} {}\r\n", self.http_ver, self.status_code, self.status_msg);

		for hd in self.header.iter() {
			res += format!("{}: {}\r\n", hd.key, hd.value).as_str();
		}

		res += "Content-Length: ";
		res += self.content_length.to_string().as_str();
		res += "\r\n\r\n";

		Ok(res)
	}

	pub fn build_content(&self) -> &[u8] {
		self.content.as_slice()
	}
}
