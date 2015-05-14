use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("show-action", Some(m)) => {
            // PLACEHOLDER
            println!("Retrieving action id: {}", m.value_of("action_id").unwrap());
            Ok(())
        },
        ("show", Some(m))        => {
            // PLACEHOLDER
            println!("Retrieving acount info");
            Ok(())

        },
        _                        => Err(CliError::NoCommand)
    }
}
