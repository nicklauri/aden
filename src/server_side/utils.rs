/**
 *	Utility funtions for server_side modules.
 */

use std::io::{Error, ErrorKind};
use std::time::{Instant, Duration};
use std::path::PathBuf;
// use server_side::config::Configuration;

#[derive(Debug)]
pub struct Timer {
	is_init: bool,
	start: Instant
}

impl Timer {
	pub fn new() -> Timer {
		Timer {is_init: true, start: Instant::now()}
	}

	pub fn elapsed(&self) -> Result<f64, String> {
		if !self.is_init {
			Err("Timer has not been initialized yet.".to_string())
		}
		else {
			let elapsed: Duration = self.start.elapsed();
			let time_elapsed: f64 = (elapsed.as_secs() * 1000) as f64 +
				elapsed.subsec_nanos() as f64 * 0.000001;
				// elapsed.subsec_nanos() as f64 * 0.000000001;
			Ok(time_elapsed)
		}
	}

	pub fn elapsed_in_millis(&self) -> Result<f64, String> {
		self.elapsed()
	}

	pub fn elapsed_in_second(&self) -> Result<f64, String> {
		if !self.is_init {
			Err("Timer has not been initialized yet.".to_string())
		}
		else {
			let elapsed: Duration = self.start.elapsed();
			let time_elapsed: f64 = elapsed.as_secs() as f64 +
				elapsed.subsec_nanos() as f64 * 0.000000001;
			Ok(time_elapsed)
		}
	}

	pub fn elapsed_from(other: &Timer) -> Result<f64, String> {
		if !other.is_init {
			Err("Timer in argument has not been initialized yet".to_string())
		}
		else {
			let elapsed: Duration = other.start.elapsed();
			let time_elapsed: f64 = elapsed.as_secs() as f64 +
				elapsed.subsec_nanos() as f64 * 0.000000001;
			Ok(time_elapsed)
		}
	}

	pub fn delete(self) {}
}

pub fn result_err(msg: &str) -> Result<(), Error> {
	Err(Error::new(ErrorKind::Other, msg))
}

// Use 2 func to release
pub fn get_root_path() -> String {
	use std::env;
	let mut root_path = PathBuf::from(env::args().collect::<Vec<String>>()[0].to_owned());
	// root_path.pop();
	root_path.pop();
	root_path.push(".");
	root_path.to_str().unwrap().to_string()
}

// http path to windows path
pub fn to_root_path(p: &str, root_path: &String) -> String {
	let windows_path = p.replace("/", r"\");
	root_path.to_string() + windows_path.as_str()
}
