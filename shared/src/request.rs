pub struct Request {
    request_type: u16,
    body: String
}

pub struct RequestType {}

impl RequestType {
    const GET: u16 = 100;
    const SET: u16 = 200;
    const DELETE: u16 = 300;
}