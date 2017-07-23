#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![macro_use]

/**
 *	KPenter SERVER version 0.1
 *	Author: Nick Lauri
 *	Email: khoanta.96@gmail.com
 *	Copyright (c) 2017 by Nick Lauri.
 */

use std::io::Read;

mod server_side;
mod client_side;

use server_side::config::Configuration;

fn main() {
	let config = match Configuration::new() {
		Ok(c) => c,
		Err(e) => {
			println!("E: Can't start the server because of: {}", e.to_string());
			return;
		}
	};

	let server_address = config.get_value_or("server_address", "localhost");
	let server_port = config.get_value_or("server_port", "8080");
	let server = match server_side::Server::new(server_address.as_str(), server_port.as_str()) {
		Ok(s) => s,
		Err(e) => {
			println!("The server can't start because: {}", e.to_string());
			return;
		}
	};

	println!("The server is running @ {}:{} .", server_address, server_port);
	std::thread::spawn(move || {
		server.start_with_thread(&config);
	});

	println!("press enter or Ctrl-C to quit.");

	// Need module command.
	loop {
		let mut line = String::new();
		std::io::stdin().read_line(&mut line);
		if line.trim().starts_with("quit") {
			println!("Exiting...");
			std::process::exit(0);
		}
	}
}
