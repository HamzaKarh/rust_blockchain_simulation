#![feature(proc_macro_hygiene, decl_macro)]

use std::os::unix::raw::time_t;
use std::{thread, time};
use std::io::{stdin, stdout};
use rocket::{self, get, routes};
use crate::blockchain::Blockchain;

mod blockchain;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



fn main() {


    let mut b:Blockchain = Blockchain::new();
    let handle = thread::spawn( move || { b.start_node(); });
    rocket::ignite()
        // .attach(DbConn::fairing())
        .mount("/", routes![index])
        // .mount("/files", routes![create_file, get_all_files])
        .launch();
    handle.join().unwrap();

}

