/**
 *
 */
pub mod config;
pub mod mimetype;
pub mod response;
pub mod status;
pub mod utils;

use std::fs::metadata;
use std::io;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::io::{Error, ErrorKind};
use std::net;
use std::net::{IpAddr, ToSocketAddrs};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use client_side::request::Request;
use server_side::{config::Configuration, mimetype::Mimetype, response::Response};

// use string to be easy to combine and no need to convert from int
#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub port: String,
    server: TcpListener,
    mimetype: Mimetype,
}

impl Server {
    pub fn new(server_address: &str, server_port: &str) -> Result<Server, Error> {
        let ip_addrs: Vec<IpAddr> = match (server_address, 0)
            .to_socket_addrs()
            .map(|iter| iter.map(|socket_address| socket_address.ip()).collect())
        {
            Ok(ip_vec) => ip_vec,
            Err(e) => return Err(e),
        };
        let ipv4 = ip_addrs[ip_addrs.len() - 1].to_string();

        if let Err(e) = server_port.parse::<i32>() {
            return Err(Error::new(ErrorKind::Other, e.to_string()));
        }

        match TcpListener::bind(ipv4.to_owned() + ":" + server_port) {
            Ok(tcplistener) => Ok(Server {
                address: server_address.to_string(),
                port: server_port.to_string(),
                server: tcplistener,
                mimetype: match Mimetype::new() {
                    Ok(mt) => mt,
                    Err(e) => {
                        // println!("server_side::Server::new -> init mimetype failed: {:?}", e.to_string());
                        return Err(e);
                    }
                },
            }),
            Err(e) => Err(e),
        }
    }

    // pub fn set_nonblocking(&self, mode: bool) -> Result<(), io::Error> {
    // 	self.server.set_nonblocking(mode)
    // }

    pub fn start_with_thread(self, config: &Configuration) {
        let max_alive_thread: u32 = match config.get_value_or("max_alive_thread", "8").parse() {
            Ok(ok) => ok,
            Err(_) => 10,
        };

        loop {
            match self.server.accept() {
                Ok((socket, sock_addr)) => {
                    let mimetype = self.mimetype.to_owned();
                    let cloned_config = config.to_owned();
                    thread::spawn(move || {
                        let mut sock = match socket.try_clone() {
                            Ok(sock) => sock,
                            Err(e) => {
                                println!("Cloning error by: {}", e.to_string());
                                return;
                            }
                        };
                        // Server::handle_client(&mut sock);
                        Server::handle_client(
                            &mut sock,
                            sock_addr.ip().to_string(),
                            mimetype,
                            cloned_config,
                        );
                    });
                }
                Err(e) => {
                    println!("Connect error by: {}", e.to_string());
                }
            }
        }
    }

    fn handle_client(
        client: &mut TcpStream,
        ip: String,
        mimetype: Mimetype,
        config: Configuration,
    ) {
        // TODO: use client_side module, handle request, send response
        // set read time-out for client
        let tcp_read_timeout = Duration::from_millis(1000);
        let home_dir = config.get_value_or("home_dir", "/www");
        let home_dir_err = config.get_value_or("home_dir_err", "/error");
        let default_index_file = config.get_value_or("default_index_file", "index.html");
        let alternative_index_basename = config.get_value_or("alternative_index_basename", "index");
        let forbidden_dirs_raw = config.get_value_or("forbidden_dir", "");
        let forbidden_dirs = forbidden_dirs_raw.split(";").collect::<Vec<&str>>();
        let timer = utils::Timer::new();
        let tcp_read_timeout: Option<Duration>;
        let tcpstream_nonblocking = match config
            .get_value_or("tcpstream_nonblocking", "false")
            .as_str()
        {
            "true" => true,
            _ => false,
        };

        // client.set_nonblocking(tcpstream_nonblocking);

        match config.get_value_or("tcp_read_timeout", "0").parse::<u64>() {
            Ok(millis) => {
                if millis != 0 {
                    tcp_read_timeout = Some(Duration::from_millis(millis));
                } else {
                    tcp_read_timeout = None;
                }
            }
            Err(_) => tcp_read_timeout = None,
        };

        client.set_read_timeout(tcp_read_timeout);
        // println!("Receiving data ...");

        let mut req_raw_header: Vec<u8> = vec![];
        let mut buff_u8_1 = [0; 1];
        let mut buff_u8 = [0; 1000];
        let break_loop = false;

        match client.read(&mut buff_u8_1[..]) {
            Ok(_) => {
                client.set_nonblocking(tcpstream_nonblocking);
                req_raw_header.push(buff_u8_1[0]);
            }
            Err(e) => {
                // println!("{} - 401 - <null> `{}`", ip, e.to_string());
                println!(
                    "{} - 401 - <null> (Timeout) {}ms",
                    ip,
                    timer.elapsed().unwrap() as f32
                );
                client.shutdown(net::Shutdown::Both);
                return;
            }
        }

        loop {
            let req_content_len = match client.read(&mut buff_u8[..]) {
                Ok(len) => len,
                Err(e) => {
                    // client.write_all("HTTP/1.1 408 Request Timeout\r\nServer: Aden 0.1.0\r\n\r\nClient request timed out.\r\n".as_bytes());
                    if req_raw_header.len() == 0 {
                        println!(
                            "{} - 408 - <null> {}ms",
                            ip,
                            timer.elapsed().unwrap() as f32
                        );
                        client.shutdown(net::Shutdown::Both);
                        return;
                    } else {
                        break;
                    }
                }
            };

            req_raw_header.extend(buff_u8.to_vec());

            if req_content_len != 1000 {
                break;
            }

            let rrhlen = req_raw_header.len();
            if rrhlen > 4 {
                if req_raw_header[rrhlen - 1] == '\n' as u8
                    && req_raw_header[rrhlen - 2] == '\r' as u8
                    && req_raw_header[rrhlen - 3] == '\n' as u8
                    && req_raw_header[rrhlen - 4] == '\r' as u8
                {
                    break;
                }
            }
        }

        let mut req: Request = match Request::new(&req_raw_header) {
            Ok(r) => r,
            Err(e) => {
                println!(
                    "{} - 401 - <null> (Bad request) {}ms",
                    ip,
                    timer.elapsed().unwrap() as f32
                );
                return;
            }
        };

        // unalias req.req_path before
        // check security error:
        match req.get_header("Content-Length".to_string()) {
            Ok(ct) => {
                let content_len: usize = ct.parse().unwrap();
                let mut req_raw_content: Vec<u8> = vec![0; content_len];
                client.read_exact(&mut req_raw_content);
                req.content = String::from_utf8_lossy(&req_raw_content.as_slice()).into_owned();
            }
            Err(_) => {}
        };

        let root_path = utils::get_root_path();
        let req_path_split_query_string: Vec<&str> = req.req_path.split("?").collect();
        let real_req_path = req_path_split_query_string[0];
        let query_string: &str;
        if req_path_split_query_string.len() > 1 {
            // For future use.
            query_string = req_path_split_query_string[1];
        } else {
            query_string = "";
        }

        let unalias_path = home_dir.to_owned() + real_req_path;
        let mut res: Response = Response::new();
        let mut forbidden = false;
        for dir in forbidden_dirs {
            if unalias_path.starts_with(dir) {
                forbidden = true;
            }
        }

        let mut req_path = utils::to_root_path(unalias_path.as_str(), &root_path);
        let req_path_isdir = match metadata(&req_path) {
            Ok(mtdat) => mtdat.is_dir(),
            Err(_) => false,
        };

        if forbidden {
            req_path = utils::to_root_path((home_dir_err + "/403.html").as_str(), &root_path);
            res.add_content_from_file(req_path.to_string());
            res.set_response_text(Some("1.1"), Some(403), Some("Forbidden"));
            res.add_header("Server", "Aden 0.1");
            res.add_header("Content-Type", "text/html");
        }
        // else if req_path.ends_with(r"\") {
        else if req_path_isdir {
            let new_req_path = if req_path.ends_with('\\') {
                req_path.to_owned() + default_index_file.as_str()
            } else {
                req_path.to_owned() + "\\" + default_index_file.as_str()
            };

            match res.add_content_from_file(new_req_path.to_owned()) {
                Ok(_) => {
                    if !req_path.to_owned().ends_with('\\') && !req.req_path.ends_with("/") {
                        let new_location = req.req_path.to_owned() + "/";
                        println!("new_location: {}", new_location);
                        res.set_response_text(Some("1.1"), Some(301), Some("Moved Permanently"));
                        res.add_header("Location", new_location.as_str());
                    } else {
                        res.set_response_text(Some("1.1"), Some(200), Some("OK"));
                    }

                    res.add_header("Server", "Aden 0.1");
                    res.add_header(
                        "Content-Type",
                        mimetype.get_mimetype_or(&req_path, "text/html").as_str(),
                    );
                    req_path = new_req_path;
                }
                Err(e) => {
                    req_path =
                        utils::to_root_path((home_dir_err + "/404.html").as_str(), &root_path);
                    res.add_content_from_file(req_path.to_string());
                    res.set_response_text(Some("1.1"), Some(404), Some("Not Found"));
                    res.add_header("Server", "Aden 0.1");
                    res.add_header("Content-Type", "text/html");
                }
            }
        } else {
            // must check alias path and convert before send response
            match res.add_content_from_file(req_path.to_owned()) {
                Ok(_) => {
                    res.set_response_text(Some("1.1"), Some(200), Some("OK"));
                    res.add_header("Server", "Aden 0.1");
                    res.add_header(
                        "Content-Type",
                        mimetype.get_mimetype_or(&req_path, "text/html").as_str(),
                    );
                }
                Err(e) => {
                    // println!("E: Can't response because: {}", e.to_string());
                    req_path =
                        utils::to_root_path((home_dir_err + "/404.html").as_str(), &root_path);
                    res.add_content_from_file(req_path.to_string());
                    res.set_response_text(Some("1.1"), Some(404), Some("Not Found"));
                    res.add_header("Server", "Aden 0.1");
                    res.add_header("Content-Type", "text/html");
                }
            }
        }

        let res_built_hd = match res.build_header() {
            Ok(r) => r,
            Err(e) => {
                println!("E: Can't build response by: {}", e.to_string());
                return;
            }
        };

        client.set_nonblocking(false);
        client.write_all(res_built_hd.as_bytes());

        // Fix this res.build_content if file size is too big, crash system.
        let mut client_bufwriter = BufWriter::new(client);
        loop {
            let (content, remaining_bytes) = res.build_content();
            client_bufwriter.write(content);
            client_bufwriter.flush();
            if remaining_bytes == 0 {
                break;
            }
        }

        println!(
            "{} - {} - {} ({} ms)",
            ip,
            res.get_status_code(),
            req.req_path,
            timer.elapsed().unwrap() as f32
        );
    }

    pub fn shutdown(self) {}
}
