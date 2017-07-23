use std::path;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{Error, ErrorKind};

use server_side::utils;

#[derive(Debug)]
pub struct Configuration(Vec<ConfUnit>);

#[derive(Debug)]
pub struct ConfUnit(pub String, pub String);

impl Configuration {
	pub fn new() -> Result<Configuration, Error> {
		let mut config: Vec<ConfUnit> = vec![];
		let root_path = utils::get_root_path();
		let config_pathbuf = path::PathBuf::from(utils::to_root_path("/config/config.conf", &root_path));
		let config_path: &str = config_pathbuf.to_str().unwrap();
		let config_file_handle = match File::open(config_path) {
			Ok(ok) => ok,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("can't find `{:?}`", config_path)))
		};
		let config_bufreader = BufReader::new(&config_file_handle);

		let mut line_num: usize = 0;
		for line_unwrap in config_bufreader.lines() {
			line_num += 1;
			let line_raw = line_unwrap.unwrap();
			let line = line_raw.trim();

			if line.starts_with("#") || line.len() == 0 {
				continue;
			}

			let mut split: Vec<&str> = line.split("=").collect();
			if split.len() < 2 {
				println!("E: config file has error on line {}: `{}`", line_num, line_raw);
				continue;
			}

			let key = split[0].trim().to_string();
			split.remove(0);
			let value = split.join("").trim().to_string();

			for cu in config.iter_mut() {
				if cu.0 == key {
					cu.1 = value.to_owned();
					break;
				}
			}

			config.push(ConfUnit(key, value));
		}

		Ok(Configuration(config))
	}

	pub fn to_owned(&self) -> Configuration {
		let mut config: Vec<ConfUnit> = vec![];

		for cu in self.0.iter() {
			config.push(ConfUnit(cu.0.to_owned(), cu.1.to_owned()));
		}

		Configuration(config)
	}

	pub fn get_value_or(&self, key: &str, default: &str) -> String {
		let key_string = key.to_string();
		for cu in self.0.iter() {
			if key == cu.0 {
				return cu.1.to_owned();
			}
		}

		default.to_string()
	}
}
