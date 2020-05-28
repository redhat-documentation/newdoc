extern crate clap;
use clap::App;

fn main() {
    println!("Hello, world!");

    App::new("newdoc")
        .version("v2.0.0")
        .get_matches();
}
