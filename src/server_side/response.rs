/**
 *
 */

use std::fs::{File, Metadata, metadata};
use std::io::{Read, BufReader};
use std::io::{Error, ErrorKind};

pub struct Response {
	http_ver: String,
	status_code: u16,
	status_msg: String,
	header: Vec<HeaderData>,
	content: Vec<u8>,
	content_length: u64,

	file_path: String,
	bufreader: Option<BufReader<File>>,
	read_buffer_size: u64,	// file > 5MiB (5000, customized in /config) will be treated as a big file ;)
	bytes_read: u64,
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
			file_path: "".to_string(),
			bufreader: None,
			read_buffer_size: 0,
			bytes_read: 0,
			is_ready: false
		}
	}

	pub fn add_header(&mut self, k: &str, v: &str) {
		// Don't allow to change content-length
		if k.to_lowercase() == "content-length" {
			return;
		}

		// No duplicate, just override
		// If match, move it and return.
		for hd in self.header.iter_mut() {
			if hd.key.to_lowercase() == k.to_lowercase() {
				hd.value = v.to_string();
				return;
			}
		}

		self.header.push(HeaderData{ key: k.to_string(), value: v.to_string() });
	}

	// Safe method, delete if exist and do nothing if not found.
	pub fn remove_header(&mut self, k: String) {
		if let Some(index) = self.header.iter()
			.position(|x| *x.key.to_lowercase() == k.to_lowercase()) {
			self.header.remove(index);
		}
	}

	pub fn set_response_text(&mut self, h: Option<&str>, c: Option<u16>, m: Option<&str>) {
		if let Some(http_ver) = h {
			self.http_ver = http_ver.to_string();
		}

		if let Some(status_code) = c {
			self.status_code = status_code;
		}

		if let Some(status_msg) = m {
			self.status_msg = status_msg.to_string();
		}
	}

	pub fn add_content(&mut self, filepath: String) -> Result<(), Error> {
		self.add_content_from_file(filepath)
	}

	// Enter file path!
	pub fn add_content_from_string(&mut self, content: String) {
		if !content.is_empty() {
			self.content_length = content.len() as u64;
			self.content = content.as_bytes().to_vec();
		}
	}

	pub fn add_content_from_file(&mut self, filepath: String) -> Result<(), Error> {
		let fp: File = File::open(filepath.to_owned())?;
		let mut fp_bufreader = BufReader::new(fp);
		let file_size = metadata(&filepath).unwrap().len();

		self.bufreader = Some(fp_bufreader);
		self.file_path = filepath;
		self.content_length = file_size;
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

	// Return: contents + size of the rest of contents.
	pub fn build_content(&mut self) -> (&[u8], u64) {
		// Destroy old contents, prepare for new contents.
		// self.content = vec![];
		
		// Buffer size must be over 5MiB to read less times.
		if self.read_buffer_size < 1_000_000 {
			// self.read_buffer_size = 2_000_000;
			self.read_buffer_size = 5_000_000;
		}

		let mut u8_buff: Vec<u8>;
		let mut numbytes: u64;
		if self.content_length - self.bytes_read < self.read_buffer_size {
			numbytes = self.content_length - self.bytes_read;
			u8_buff = vec![0; numbytes as usize];
		}
		else {
			numbytes = self.read_buffer_size;
			u8_buff = vec![0;  numbytes as usize];
		}

		if let Some(ref mut br) = self.bufreader {
			if let Ok(_) = br.read_exact(&mut u8_buff[..]) {
				self.bytes_read += numbytes as u64;
			}
		}

		// println!("bytes read: {}", self.bytes_read);

		self.content = u8_buff;

		// Ok((self.content.as_slice(), self.content_length - self.bytes_read))
		(self.content.as_slice(), self.content_length - self.bytes_read)
	}

	pub fn get_status_code(&self) -> u16 {
		self.status_code
	}

	pub fn get_content_length(&self) -> u64 {
		self.content_length
	}
}
