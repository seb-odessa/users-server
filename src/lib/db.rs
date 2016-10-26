use rusqlite::{Connection, Error};
use std::path::Path;
use messages::{UserDesc, Response, ResponseStatus};
use time;

const USERS_DB: &'static str = "users.db";

pub enum CMD {
    CREATE,
    REMOVE,
    LOGIN
}

pub fn process(cmd : CMD, user : &UserDesc) -> Response {
    match cmd {
        CMD::CREATE => execute(&create, &user),
        CMD::REMOVE => execute(&remove, &user),
        CMD::LOGIN  => execute(&login, &user),
    }
}

fn execute(f: &Fn(&UserDesc) -> Result<String, Error>, user: &UserDesc) -> Response {
    match f(&user) {
        Result::Ok(msg) => {
            Response {
                status: ResponseStatus::OK,
                desc: msg,
            }
        }
        Result::Err(err) => {
            Response {
                status: ResponseStatus::ERROR,
                desc: format!("{}", err),
            }
        }
    }
}

fn create(user: &UserDesc) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    try!(conn.execute("INSERT INTO users (name, password, created) VALUES ($1, $2, $3)",
                      &[&user.name, &user.password, &time::get_time()]));
    Ok(format!("The user '{}' was created", user.name))
}

fn remove(user: &UserDesc) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    try!(conn.execute("DELETE FROM users WHERE name=? AND password=?",
                      &[&user.name, &user.password]));
    Ok(format!("The user '{}' was removed", user.name))
}

fn login(user: &UserDesc) -> Result<String, Error> {
    let conn = try!(Connection::open(Path::new(USERS_DB)));
    let users: i32 = try!(conn.query_row("SELECT COUNT(id) FROM users WHERE name=?",
                                         &[&user.name],
                                         |row| row.get(0)));
    if 0 == users {
        return Err(Error::InvalidParameterName(format!("The user '{}' was not found", user.name)));
    }

    let pass: i32 = try!(conn.query_row("SELECT COUNT(id) FROM users WHERE name=? AND password=?",
                                        &[&user.name, &user.password],
                                        |row| row.get(0)));
    if 0 == pass {
        return Err(Error::InvalidParameterName(format!("Invalid password for '{}'", user.name)));
    }

    Ok(format!("The user '{}' was logged in", user.name))
}
