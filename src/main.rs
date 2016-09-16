extern crate rustc_serialize;
extern crate time;
extern crate rusqlite;

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};
use rusqlite::{Connection, Error};
use std::path::Path;
use rustc_serialize::json;

const USERS_DB :&'static str = "users.db";

#[derive(RustcDecodable, RustcEncodable)]
struct UserAuth {
    name : String,
    password : String
}

#[derive(RustcDecodable, RustcEncodable)]
enum ResponseStatus{OK, ERROR}

#[derive(RustcDecodable, RustcEncodable)]
struct Response {
    status : ResponseStatus,
    desc : String
}

fn main() {

    let mut server = Nickel::new();
    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:id", middleware! { |request| format!("This is user: {:?}", request.param("id"))  });
    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));

    server.post("/user/create", middleware! { |request, response|
        let user = request.json_as::<UserAuth>().unwrap();
        json::encode(&process(&create, &user)).unwrap()
    });

    server.post("/user/remove", middleware! { |request, response|
        let user = request.json_as::<UserAuth>().unwrap();
        json::encode(&process(&remove, &user)).unwrap()
    });

    server.post("/user/login", middleware! { |request, response|
        let user = request.json_as::<UserAuth>().unwrap();
        json::encode(&process(&login, &user)).unwrap()
    });

    server.listen("127.0.0.1:6000");
}

fn process(f: &Fn(&UserAuth)->Result<String, Error>, user : &UserAuth) -> Response {
    match f(&user) {
        Result::Ok(msg) => Response{ status : ResponseStatus::OK, desc : msg },
        Result::Err(err) => Response{ status : ResponseStatus::ERROR, desc : format!("{}", err) }
    }
}

fn create(user : &UserAuth) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    try!(conn.execute("INSERT INTO users (name, password, created) VALUES ($1, $2, $3)",
                      &[&user.name, &user.password, &time::get_time()]));
    Ok(format!("The user '{}' was created", user.name))
}

fn remove(user : &UserAuth) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    try!(conn.execute("DELETE FROM users WHERE name=? AND password=?",
                      &[&user.name, &user.password]));
    Ok(format!("The user '{}' was removed", user.name))
}

fn login(user : &UserAuth) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    let users : i32 = try!(conn.query_row("SELECT COUNT(id) FROM users WHERE name=?",
                        &[&user.name],
                        |row| { row.get(0) }));
    if 0 == users {
        return Err(Error::InvalidParameterName(format!("The user '{}' was not found", user.name)));
    }

    let pass : i32 = try!(conn.query_row("SELECT COUNT(id) FROM users WHERE name=? AND password=?",
                                          &[&user.name, &user.password],
                                          |row| { row.get(0) }));
    if 0 == pass {
        return Err(Error::InvalidParameterName(format!("Invalid password for '{}'", user.name)));
    }

    Ok(format!("The user '{}' was logged in", user.name))
}


