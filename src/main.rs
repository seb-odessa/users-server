extern crate rustc_serialize;
extern crate time;
extern crate sqlite3;

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};
use time::Timespec;

#[derive(RustcDecodable, RustcEncodable)]
struct UserAuth {
    name: String,
    password: String,
}

/*
#[derive(Debug)]
struct UserRecord {
    id: u32,
    name: String,
    password: String,
    created: Timespec
}
*/

fn main() {

    let mut server = Nickel::new();
    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:id", middleware! { |request| format!("This is user: {:?}", request.param("id"))  });
    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));

    server.post("/user/create", middleware! { |request, response|
        let user = request.json_as::<UserAuth>().unwrap();
        format!("Create user {} with password {}", user.name, user.password)
    });

    server.post("/user/remove", middleware! { |request, response|
        let user = request.json_as::<UserAuth>().unwrap();
        format!("Remove user {} with password {}", user.name, user.password)
    });

    server.listen("127.0.0.1:6000");
}
