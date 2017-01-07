#![feature(alloc_system)]
extern crate alloc_system;

extern crate nix;
extern crate libc;

mod utils;
mod events;
mod environment;

fn main() {
    println!("PliOS init initalizing...");

    environment::init_environment();
    environment::init_api_filesystems();

    events::handle_events();
}
