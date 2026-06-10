#[allow(dead_code)]
pub struct Request;

#[allow(dead_code)]
impl Request {
    pub const CHANGE_PROG: u8 = 0x02;
    pub const RESOLUTION: u8 = 0x0c;
    pub const VERSION: u8 = 0x0e;
    pub const BRIGHTNESS: u8 = 0x10;
    pub const SERVO: u8 = 0x12;
    pub const LED: u8 = 0x14;
    pub const LAMP: u8 = 0x16;
    pub const FPS: u8 = 0x18;
}

#[allow(dead_code)]
pub struct Response;

#[allow(dead_code)]
impl Response {
    pub const RESOLUTION: u8 = 0x0d;
    pub const VERSION: u8 = 0x0f;
    pub const RESULT: u8 = 0x01;
    pub const ERROR: u8 = 0x03;
}
