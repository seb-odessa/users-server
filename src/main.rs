extern crate lib;

use lib::api;

const API_URL: &'static str = "127.0.0.1:6000";

fn main() {
    api::run(API_URL);
}

