#[derive(Debug)]
pub struct HttpStatus(u16, &'static str);

impl HttpStatus {
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.0, self.1)
    }
}

/// 	Define some status code
pub const REQUEST_OK: HttpStatus = HttpStatus(200, "OK");

pub const BAD_REQUEST: HttpStatus = HttpStatus(400, "Bad Request");
pub const FORBIDDEN: HttpStatus = HttpStatus(403, "Forbidden");
pub const NOT_FOUND: HttpStatus = HttpStatus(404, "Not Found");

pub const INTERNAL_SERVER_ERROR: HttpStatus = HttpStatus(500, "Internal Server Error");
