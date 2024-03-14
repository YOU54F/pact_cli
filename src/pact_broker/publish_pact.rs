//! CLI to publish provider contracts to a Pact broker.

#![warn(missing_docs)]

use std::fs::File;
use anyhow::Context;
use log::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use clap::Args;
use pact_cli::setup_loggers;

#[derive(Debug, Args)]
pub struct PactPublisherArgs {
    #[clap(short='l', long="loglevel", value_parser=clap::builder::PossibleValuesParser::new(&["error", "warn", "info", "debug", "trace", "none"]), help="Log level (defaults to warn)")]
    loglevel: Option<String>,

    #[clap(
        short = 'b',
        long = "broker-base-url",
        number_of_values = 1,
        value_parser = clap::builder::NonEmptyStringValueParser::new(),
        required = true,
        env = "PACT_BROKER_BASE_URL",
        help = "The base URL of your Pact Broker - can be set with the environment variable PACT_BROKER_BASE_URL"
    )]
    broker_base_url: Option<String>,

        #[clap(
        // short = 't',
        long = "broker-token",
        number_of_values = 1,
        value_parser = clap::builder::NonEmptyStringValueParser::new(),
        required = false,
        env = "PACT_BROKER_TOKEN",
        help = "Bearer token to use to publish with - can be set with the environment variable PACT_BROKER_TOKEN"
    )]
    broker_token: Option<String>,


}



pub fn main(args: PactPublisherArgs) -> Result<(), i32> {
    let log_level = args.loglevel.clone();
    if let Err(err) = setup_loggers(&log_level.unwrap_or("warn".to_string())) {
        eprintln!("WARN: Could not setup loggers: {}", err);
        eprintln!();
    }
    print!("Pact Publisher Args: {:?}", args);
    Ok(())
}
