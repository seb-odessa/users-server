use rusqlite::{Connection, Error};
use messages::{UserDesc, Response, Status, CMD};

const USERS_DB: &'static str = "users.db";

pub fn process(cmd : CMD, user : &UserDesc) -> Response {
    match connect() {
        Ok(conn) => {
            match cmd {
                CMD::CREATE => user_create(&conn, &user),
                CMD::REMOVE => user_remove(&conn, &user),
                CMD::EXISTS => user_exists(&conn, &user),
            }
        },
        Err(err) => Response {
            status: Status::ERROR,
            desc: format!("{}", err),
        }
    }
}

fn connect() -> Result<Connection, Error> {
    use std::path::Path;
    Connection::open(Path::new(USERS_DB))
}

fn user_create(conn: &Connection, user: &UserDesc) -> Response {
    use time;
    let sql = "INSERT INTO users (name, password, created) VALUES ($1, $2, $3)";
    match conn.execute(sql, &[&user.name, &user.password, &time::get_time()]) {
        Ok(_) => Response {
            status: Status::OK,
            desc: format!("The user '{}' was created", user.name)
        },
        Err(err) => Response {
            status: Status::ERROR,
            desc: format!("{}", err),
        }
    }
}

fn user_remove(conn: &Connection, user: &UserDesc) -> Response {
    let exist = user_exists(&conn, &user);
    if Status::ERROR == exist.status  {
        return exist;
    }

    let sql = "DELETE FROM users WHERE name=? AND password=?";
    match conn.execute(sql,  &[&user.name, &user.password]) {
        Ok(_) => Response {
            status: Status::OK,
            desc: format!("The user '{}' was removed", user.name)
        },
        Err(err) => Response {
            status: Status::ERROR,
            desc: format!("{}", err),
        }
    }
}

fn user_exists(conn: &Connection, user: &UserDesc) -> Response {
    let sql = "SELECT COUNT(id) FROM users WHERE name=? AND password=?";
    match conn.query_row(sql, &[&user.name, &user.password], |row| row.get(0)) {
        Ok(users) => {
            if 1 == users {
                Response {
                    status: Status::OK,
                    desc: format!("The user '{}' exist", user.name)
                }
            }  else {
                Response {
                    status: Status::ERROR,
                    desc: format!("The user '{}' does not exist or incorrect password", user.name)
                }
            }
        },
        Err(err) => Response {
            status: Status::ERROR,
            desc: format!("{}", err),
        }
    }
}
