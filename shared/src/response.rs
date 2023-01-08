pub struct StatusCode {}

impl StatusCode {
    pub const OK: u16 = 100;
    pub const BAD_REQUEST: u16 = 200;
    pub const INTERNAL_ERROR: u16 = 300;
}

pub struct Response {
    status_code: StatusCode,
    body: String
}