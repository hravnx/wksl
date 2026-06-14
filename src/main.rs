use std::io::{stdout, Write};
use std::net::Ipv4Addr;
use std::process::Command;

use clap::{Parser, Subcommand};
use config::RunError;

mod config;
mod wake;

/// Used when no `port` has been given by the user in the config file
const DEFAULT_WOL_PORT: u16 = 9;

/// Used when no `broadcast_address` has been given by the user in the config file
const DEFAULT_BROADCAST: Ipv4Addr = Ipv4Addr::BROADCAST;

const PROGRAM_NAME: &str = "wksl";
const CONFIG_PATH: &str = "~/.config/wksl/config.toml";

#[derive(Parser, Debug)]
#[command(
    name = PROGRAM_NAME,
    bin_name = PROGRAM_NAME,
    version,
    infer_subcommands = true,
    arg_required_else_help = true
)]
struct Cli {
    /// Path to config file
    #[arg(short = 'c', long = "config-path", default_value = CONFIG_PATH)]
    config_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Broadcasts a wake-on-lan package to a mac address
    Wake {
        /// Selects the machine to use
        machine: String,
    },

    /// Suspends the machine via ssh
    Sleep {
        /// Selects the machine to use
        machine: String,
    },
}

fn run_app(args: Cli) -> Result<(), RunError> {
    match args.command {
        Commands::Wake { machine } => {
            let config = config::read_config(&args.config_path, &machine)?;

            print!("Waking '{}' up ... ", machine);
            stdout().flush()?;

            let packet = wake::make_packet(&config.mac_address.bytes());
            let port = config.port.unwrap_or(DEFAULT_WOL_PORT);
            let cast_to = config
                .broadcast_address
                .unwrap_or_else(|| DEFAULT_BROADCAST.into());
            wake::send_packet(&packet, cast_to, port).map(|_| println!("done."))
        }
        Commands::Sleep { machine } => {
            let config = config::read_config(&args.config_path, &machine)?;

            if let Some(sleep_cmd) = config.sleep_command {
                print!("Putting '{}' to sleep ... ", machine);
                stdout().flush()?;
                let output = Command::new(sleep_cmd.cmd).args(sleep_cmd.args).output()?;
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
    }
}


fn main() {
    // match command line arguments
    let args = Cli::parse();

    // run the program
    std::process::exit(match run_app(args) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("ERROR: {:#?}", err);
            1
        }
    });
}
