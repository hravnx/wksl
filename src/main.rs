use std::io::{stdout, Write};
use std::net::Ipv4Addr;
use std::process::Command;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand, crate_version};
use config::RunError;

mod config;
mod wake;

/// Used when no `port` has been given by the user in the config file
const DEFAULT_WOL_PORT: u16 = 9;

/// Used when no `broadcast_address` has been given by the user in the config file
const DEFAULT_BROADCAST: Ipv4Addr = Ipv4Addr::BROADCAST;

const PROGRAM_NAME: &str = "wksl";
const CONFIG_PATH: &str = "~/.config/wksl/config.toml";

fn run_app(matches: ArgMatches) -> Result<(), RunError> {
    // The CONFIG_PATH option has a default value, so unwrap will never panic here
    let config_path = matches.value_of("CONFIG-PATH").unwrap();

    if let Some(matches) = matches.subcommand_matches("wake") {
        // The MACHINE option is marked as required, so unwrap will never panic here
        let machine = matches.value_of("MACHINE").unwrap();
        let config = config::read_config(config_path, machine)?;

        print!("Waking '{}' up ... ", machine);
        stdout().flush()?;

        let packet = wake::make_packet(&config.mac_address.bytes());
        let port = config.port.unwrap_or(DEFAULT_WOL_PORT);
        let cast_to = config
            .broadcast_address
            .unwrap_or_else(|| DEFAULT_BROADCAST.into());
        return wake::send_packet(&packet, cast_to, port)
            .map(|_| println!("done."));
    }

    if let Some(matches) = matches.subcommand_matches("sleep") {
        // The MACHINE option is marked as required, so unwrap will never panic here
        let machine = matches.value_of("MACHINE").unwrap();
        let config = config::read_config(config_path, machine)?;

        return if let Some(sleep_cmd) = config.sleep_command {
            print!("Putting '{}' to sleep ... ", machine);
            stdout().flush()?;
            let output =
                Command::new(sleep_cmd.cmd)
                    .args(sleep_cmd.args)
                    .output()?;
            if output.status.success() {
                println!("done.");
                Ok(())
            } else {
                eprintln!("failed.");
                std::io::stderr().write_all(&output.stderr)?;
                Err(RunError::SleepCommandFailed)
            }
        } else {
            Err(RunError::NoSleepCommand)
        }
    }

    unreachable!("Unhandled sub command {:#?}", matches)
}


fn main() {
    // match command line arguments
    let matches = App::new(PROGRAM_NAME)
        .bin_name(PROGRAM_NAME)
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("CONFIG-PATH")
            .short("c")
            .long("config-path")
            .help("Path to config file")
            .default_value(CONFIG_PATH))
        .subcommand(SubCommand::with_name("wake")
            .about("Broadcasts a wake-on-lan package to a mac address")
            .arg(Arg::with_name("MACHINE")
                .help("Selects the machine to use")
                .required(true)))
        .subcommand(SubCommand::with_name("sleep")
            .about("Suspends the machine via ssh")
            .arg(Arg::with_name("MACHINE")
                .help("Selects the machine to use")
                .required(true)))
        .get_matches();

    // run the program
    std::process::exit(match run_app(matches) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {:#?}", err);
            1
        }
    });
}
