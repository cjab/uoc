pub use self::login_seed::LoginSeed;
pub use self::login_request::LoginRequest;
pub use self::server_list::ServerList;

use std::io;
use std::string;
use byteorder;

mod login_seed;
mod login_request;
mod server_list;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(string::FromUtf8Error),
    ByteOrder(byteorder::Error)
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}


impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::ByteOrder(err)
    }
}


impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}


pub trait Packet {
    fn packet_id() -> u8;
    fn bytes(&self) -> Vec<u8>;
}
