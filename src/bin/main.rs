use clap::{clap_app, crate_authors};
use dotenv::dotenv;

use findip_lib::hello_world;
use findip_lib::properties::load_config_from_file;

pub fn main() {
    dotenv().ok();

    let matches = clap_app!(findip =>
        (version: "1.0.0")
        (author: crate_authors!(", "))
        (about: "A lightweight utility that finds out and reports public IP addresses")
        (@arg config_file_name: -c --("config-file-name") <FILE_NAME> "A configuration file that provides information on how to notify.")
        (@arg verbose: -v --verbose +multiple "Enable verbose mode, prints debug information.")
    )
    .get_matches();

    println!("{:#?}", matches.args);
    let config =
        load_config_from_file(matches.value_of("config_file_name").unwrap().to_string()).unwrap();
    println!("{:#?}", config);

    hello_world(Option::None);
}
