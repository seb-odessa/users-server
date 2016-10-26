    
#[derive(RustcDecodable, RustcEncodable)]
pub struct UserDesc {
    pub name: String,
    pub password: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub enum ResponseStatus {
    OK,
    ERROR,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Response {
    pub status: ResponseStatus,
    pub desc: String,
}
