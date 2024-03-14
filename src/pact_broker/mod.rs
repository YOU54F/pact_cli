use clap::{Args, Parser, Subcommand};

pub mod publish_pact;
pub mod pact_broker;
pub mod utils;
#[derive(Parser, Debug)]
#[clap(author = "Pact Foundation", version, about)]
pub struct PactBrokerArguments {
    #[clap(subcommand)]
    pub cmd: PactBrokerSubCommand,
}

#[derive(Subcommand, Debug)]
pub enum PactBrokerSubCommand {
    // Pact Publish Contracts
    Publish(publish_pact::PactPublisherArgs),
}
