use clap::clap_app;

use findip_lib::hello_world;

pub fn main() {
    let matches = clap_app!(findip =>
        (version: "1.0.0")
        (author: "Yash Kulshrestha (@YashdalfTheGray)")
        (about: "A lightweight utility that finds out and reports public IP addresses")
        (@arg cron_pattern: -c --cron <PATTERN> "A cron pattern dictating how often to check the IP.")
        (@arg notifier_strategy: -n --notifier <STRATEGY> "The notifier strategy to use; one of stdout, textfile, s3, rest.")
        (@arg verbose: -v --verbose +multiple "Enable verbose mode, prints debug information.")
    ).get_matches();

    println!("{:#?}", matches.args);

    hello_world(Option::None);
}
