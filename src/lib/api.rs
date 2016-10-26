
use nickel::Request;
use nickel::{Nickel, HttpRouter, JsonBody};
use rustc_serialize::json;
use messages::UserDesc;
use db::{process, CMD};

pub fn run(url : &str) {
    let mut server = Nickel::new();
    server.post("/user/create", middleware!{ |request, response| handle(CMD::CREATE, request) });
    server.post("/user/remove", middleware!{ |request, response| handle(CMD::REMOVE, request) });
    server.post("/user/login",  middleware!{ |request, response| handle(CMD::LOGIN,  request) });
    server.listen(url);
}

fn handle(cmd : CMD, request : &mut Request) ->  String {
    match request.json_as::<UserDesc>() {
        Ok(user) => match json::encode(&process(cmd, &user)) {
            Ok(json) => json,
            Err(_) => String::from("Can't encode Response to JSON")
        },
        Err(_) => String::from("Can't parse Request JSON")
    }
}
