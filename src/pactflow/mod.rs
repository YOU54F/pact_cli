use clap::{Args, Parser, Subcommand};

pub mod publish_provider_contract;

#[derive(Parser, Debug)]
#[clap(author = "PactFlow", version, about)]
pub(crate) struct PactflowArguments {
    #[clap(subcommand)]
   pub cmd: PactflowSubCommand,
}

#[derive(Subcommand, Debug)]
pub enum PactflowSubCommand {
    // PactFlow Provider Contract Publisher
    PublishProviderContract(publish_provider_contract::PublishProviderContractArgs),
}
