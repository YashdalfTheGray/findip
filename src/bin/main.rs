use clap::{clap_app, crate_authors, Arg};
use dotenv::dotenv;

use findip_lib::hello_world;

pub fn main() {
    dotenv().ok();

    let matches = clap_app!(findip =>
        (version: "1.0.0")
        (author: crate_authors!(", "))
        (about: "A lightweight utility that finds out and reports public IP addresses")
        (@arg notifier_strategy: -n --notifier <STRATEGY> "The notifier strategy to use; one of stdout, textfile, s3, rest.")
        (@arg verbose: -v --verbose +multiple "Enable verbose mode, prints debug information.")
    ).arg(
        Arg::with_name("cron_pattern")
            .short("c")
            .long("cron")
            .value_name("PATTERN")
            .help("A cron pattern dictating how often to check the IP.")
            .takes_value(true)
    )
    .get_matches();

    println!("{:#?}", matches.args);

    hello_world(Option::None);
}
