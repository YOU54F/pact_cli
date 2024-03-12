// use std::collections::HashMap;

use std::f32::consts::E;
// use std::result::Result::Ok
use anyhow::Ok;
use clap::{Parser, Subcommand};
mod pact_broker;
mod pactflow;
#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
/// Pact CLI
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    // Pact
    PactBroker(pact_broker::PactBrokerArguments),
    // PactFlow
    Pactflow(pactflow::PactflowArguments),
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::PactBroker(_args) => {
            match _args.cmd {
                pact_broker::PactBrokerSubCommand::Publish(args) => {
                    let result = pact_broker::publish_pact::main(args);
                    print!("{:?}", result);
            }}
        }
        SubCommand::Pactflow(_args) => {
            match _args.cmd {
                pactflow::PactflowSubCommand::PublishProviderContract(args) => {
                    let result = pactflow::publish_provider_contract::main(args);
                    print!("{:?}", result);
            }}
        }
    }
}
