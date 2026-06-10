pub struct Request;
impl Request {
    pub const CHANGE_PROG: u8 = 0x02;
    pub const RESOLUTION: u8 = 0x0c;
    pub const VERSION: u8 = 0x0e;
    pub const BRIGHTNESS: u8 = 0x10;
    pub const SERVO: u8 = 0x12;
    pub const LED: u8 = 0x14;
    pub const LAMP: u8 = 0x16;
    pub const FPS: u8 = 0x18;
    pub const BLOCKS: u8 = 0x20;

    pub const NO_CHECKSUM_SYNC_L: u8 = 0xae;
    pub const NO_CHECKSUM_SYNC_H: u8 = 0xc1;
    pub const NO_CHECKSUM_SYNC: u16 = 0xc1ae;
}

pub struct Response;
impl Response {
    pub const RESOLUTION: u8 = 0x0d;
    pub const VERSION: u8 = 0x0f;
    pub const RESULT: u8 = 0x01;
    pub const ERROR: u8 = 0x03;
    pub const BLOCKS: u8 = 0x21;
}

// pub struct RequestHeader;
// impl RequestHeader {
//     // pub const BLOCKS: [u8;4] = []

// }

pub struct Signature;
impl Signature {
    pub const SIG1: u8 = 1;
    pub const SIG2: u8 = 2;
    pub const SIG3: u8 = 4;
    pub const SIG4: u8 = 8;
    pub const SIG5: u8 = 16;
    pub const SIG6: u8 = 32;
    pub const SIG7: u8 = 64;
    pub const COLOR_CODES: u8 = 128;
    pub const SIG_ALL: u8 = 0xff;
}
