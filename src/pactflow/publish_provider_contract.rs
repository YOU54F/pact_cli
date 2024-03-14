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
pub struct PublishProviderContractArgs {
    #[clap(short='l', long="loglevel", value_parser=clap::builder::PossibleValuesParser::new(&["error", "warn", "info", "debug", "trace", "none"]), help="Log level (defaults to warn)")]
    loglevel: Option<String>,
    #[clap(
        // short = 'b',
        long = "content-file",
        required = true,
        num_args = 1,
        number_of_values = 1,
        value_parser = clap::builder::NonEmptyStringValueParser::new(),
        help = "Provider specification to publish"
    )]
    content_file: Option<String>,

    #[clap(
        short = 'b',
        long = "broker-base-url",
        num_args = 1,
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
        num_args = 1,
        number_of_values = 1,
        value_parser = clap::builder::NonEmptyStringValueParser::new(),
        required = false,
        env = "PACT_BROKER_TOKEN",
        help = "Bearer token to use to publish with - can be set with the environment variable PACT_BROKER_TOKEN"
    )]
    broker_token: Option<String>,

    
#[clap(
    short = 'p',
    long = "provider",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = true,
    help = "The provider name"
)]
provider: String,

#[clap(
    short = 'a',
    long = "provider_app_version",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = true,
    // aliases = "-a",
    help = "The provider application version"
)]
provider_app_version: String,

#[clap(
    long = "branch",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    alias = "h",
    help = "Repository branch of the provider version"
)]
branch: Option<String>,

#[clap(
    short = 't',
    long = "tag",
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    alias = "t",
    num_args = 0..=1,
    help = "Tag name for provider version. Can be specified multiple times (eg. --tag v1 --tag v2)"
)]
tag: Option<Vec<String>>,

#[clap(
    long = "specification",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    default_value = "oas",
    help = "The contract specification"
)]
specification: String,

#[clap(
    long = "content_type",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The content type. eg. application/yml"
)]
content_type: Option<String>,

#[clap(
    long = "verification_success",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "Whether or not the self verification passed successfully."
)]
verification_success: bool,

#[clap(
    long = "verification_exit_code",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The exit code of the verification process. Can be used instead of --verification-success|--no-verification-success for a simpler build script."
)]
verification_exit_code: Option<u32>,

#[clap(
    long = "verification_results",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The path to the file containing the output from the verification process"
)]
verification_results: Option<String>,

#[clap(
    long = "verification_results_content_type",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The content type of the verification output eg. text/plain, application/yaml"
)]
verification_results_content_type: Option<String>,

#[clap(
    long = "verification_results_format",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The format of the verification output eg. junit, text"
)]
verification_results_format: Option<String>,

#[clap(
    long = "verifier",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The tool used to verify the provider contract"
)]
verifier: Option<String>,

#[clap(
    long = "verifier_version",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The version of the tool used to verify the provider contract"
)]
verifier_version: Option<String>,

#[clap(
    long = "build_url",
    num_args = 1,
    number_of_values = 1,
    value_parser = clap::builder::NonEmptyStringValueParser::new(),
    required = false,
    help = "The build URL that created the provider contract"
)]
build_url: Option<String>,

}

pub fn main(args: PublishProviderContractArgs) -> Result<(), i32> {
    let log_level = args.loglevel;
    if let Err(err) = setup_loggers(&log_level.unwrap_or("warn".to_string())) {
        eprintln!("WARN: Could not setup loggers: {}", err);
        eprintln!();
    }
    let content_file = load_file(&args.content_file.unwrap()).map_err(|_| 1)?;
    println!("Content file: \n\n\n{:?}", content_file);
    Ok(())
}


fn load_file(file_name: &str) -> anyhow::Result<Value> {
    let file = File::open(file_name)?;
    let file_contents = serde_yaml::from_reader(file).context("file is not JSON or YML");
    // println!("{:?}", file_contents);
    file_contents
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderContractUploadRequestBody {
    pub content: String,
    pub contract_type: String,
    pub content_type: String,
    pub verification_results: VerificationResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResults {
    pub success: String,
    pub content: String,
    pub content_type: String,
    pub verifier: String,
}
