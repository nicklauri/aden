
// TODO: Support more method. Now, just GET and POST

use std::io::Error;
use std::io::ErrorKind;
use server_side::status;
use server_side::status::HttpStatus;

#[derive(Debug)]
pub struct Request {
	pub method: String,
	pub req_path: String,
	pub http_ver: String,
	pub header: Vec<RequestData>,
	pub content: String		// for method::POST
}

// for easier, change directly.
#[derive(Debug)]
pub struct RequestData {
	pub key: String,
	pub value: String
}

impl Request {
	pub fn new(raw_req: &[u8]) -> Result<Request, HttpStatus> {
		let req_string = String::from_utf8_lossy(raw_req);

		// remove this condition and following function soon, because of performance.
		if !Request::is_valid_http_request(&req_string.to_string()) {
			return Err(status::BAD_REQUEST);
		}

		let mut lines  = req_string.lines();
		let request_line = match lines.next() {
			Some(data) => data,
			None => {
				// TODO: GET and POST must have more headers, not just request line.

				return Err(status::BAD_REQUEST);
			}
		};

		let request_line_vec: Vec<&str> = request_line.split_whitespace().collect();
		if request_line_vec.len() != 3 {
			//
			// TODO: check != 3 is bad, just join 1..(request_line_vec.len()-1)
			// TODO: show log to file or screen.
			//
			
			return Err(status::BAD_REQUEST);
		}

		let mut request_data: Vec<RequestData> = vec![];
		while let Some(line) = lines.next() {
			if line == "" {
				// reached to bottom of http request - POST data.
				break;
			}

			let mut line_vec: Vec<&str> = line.split(":").collect();
			let key = line_vec[0];

			line_vec.remove(0);
			let val_raw = line_vec.join(":");
			let val = val_raw.trim();
			let rd: RequestData = RequestData{key: key.to_string(), value: val.to_string()};
			request_data.push(rd);
		}

		let post_data = match lines.next() {
			Some(data) => data.to_string(),
			None => "".to_string()
		};

		Ok(Request {
			method: request_line_vec[0].to_lowercase(),
			req_path: request_line_vec[1].to_string(),
			http_ver: request_line_vec[2].to_string(),
			header: request_data,
			content: post_data
		})
	}

	pub fn get_header(&self, key: String) -> Result<String, Error> {
		for rd in self.header.iter() {
			if key.to_lowercase() == rd.key.to_lowercase() {
				return Ok(rd.value.to_owned());
			}
		}
		Err(Error::new(ErrorKind::Other, "field not found"))
	}

	pub fn is_valid_http_request(req: &String) -> bool {
		let lines: Vec<&str> = req.lines().collect();
		if lines.len() < 3 {
			return false;
		}

		let req_line: Vec<&str> = lines[0].split_whitespace().collect();

		if req_line.len() != 3 {
			false
		}
		else if !vec!["GET", "POST"].contains(&req_line[0].to_uppercase().as_str()) {
			// use vector to be easy to upgrade,
			// only support for GET and POST method.
			false
		}
		else {
			true
		}
	}
}
