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

    let should_decorate = config.logging_config.decorate.clone();

    {
        let this = fern::Dispatch::new().chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    if should_decorate {
                        out.finish(format_args!(
                            "{}[{}][{}] {}",
                            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                            record.target(),
                            record.level(),
                            message
                        ))
                    } else {
                        out.finish(format_args!("{}", message))
                    }
                })
                .level(config.logging_config.log_level.clone())
                .chain(std::io::stdout())
                .chain(fern::log_file(config.logging_config.log_file.clone()).unwrap()),
        );
        let (max_level, log) = this.into_log();

        log::set_boxed_logger(log)?;
        log::set_max_level(max_level);

        Ok(())
    }
    .expect("Failed to set up the fern dispatch and logging.");

    schedule_ip_notification(config);
}
