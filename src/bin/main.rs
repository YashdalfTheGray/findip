use clap::{clap_app, crate_authors};
use dotenv::dotenv;

use findip_lib::hello_world;

pub fn main() {
    dotenv().ok();

    let matches = clap_app!(findip =>
        (version: "1.0.0")
        (author: crate_authors!(", "))
        (about: "A lightweight utility that finds out and reports public IP addresses")
        (@arg cron_pattern: -c --cron <PATTERN> "A cron pattern dictating how often to check the IP.")
        (@arg notifier_strategy: -n --notifier <STRATEGY> "The notifier strategy to use; one of stdout, textfile, s3, rest.")
        (@arg verbose: -v --verbose +multiple "Enable verbose mode, prints debug information.")
    ).get_matches();

    println!("{:#?}", matches.args);

    hello_world(Option::None);
}
