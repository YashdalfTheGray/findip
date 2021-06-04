use findiplib::hello_world;

use clap::{App, Arg};

pub fn main() {
    let matches = App::new("findip")
        .version("1.0.0")
        .author("Yash Kulshrestha (@YashdalfTheGray)")
        .about("A lightweight utility that finds out and reports public IP addresses")
        .arg(
            Arg::with_name("notifier")
                .short("n")
                .long("notifier")
                .value_name("NOTIFIER")
                .help("The notifier strategy to use")
                .takes_value(true),
        )
        .get_matches();

    println!("{:?}", matches);

    hello_world(Option::None);
}
