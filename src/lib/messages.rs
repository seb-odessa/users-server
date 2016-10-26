
#[derive(RustcDecodable, RustcEncodable)]
pub enum CMD {
    CREATE,
    REMOVE,
    EXISTS,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserDesc {
    pub name: String,
    pub password: String,
}

#[derive(RustcDecodable, RustcEncodable, PartialEq)]
pub enum Status {
    OK,
    ERROR,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Response {
    pub status: Status,
    pub desc: String,
}
