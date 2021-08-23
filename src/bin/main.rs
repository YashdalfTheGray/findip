use clap::{clap_app, crate_authors};
use dotenv::dotenv;

use findip_lib::{config_file::load_config_from_file, schedule_ip_notification};

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

    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .level(config.logging_config.log_level.clone())
                .chain(std::io::stdout())
                .chain(fern::log_file(config.logging_config.log_file.clone()).unwrap()),
        )
        .apply();

    schedule_ip_notification(config);
}
