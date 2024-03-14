// use std::collections::HashMap;
mod cli;
use clap_complete::{generate_to, Shell};
use std::str::FromStr;
// use std::result::Result::Ok
// use clap::{builder::PossibleValuesParser, Arg, ArgAction, Command, Parser, Subcommand, ValueEnum};
// use clap_complete::{generate, Generator, Shell};
// use std::io;
// mod pact_broker;
// mod pactflow;
// #[derive(Parser, Debug)]
// #[command(author = "Author Name", version, about)]
// pub fn build_cli() -> Command {
//     Command::new("pact-cli")
//         .subcommand(pact_broker::build_cli())
//         .subcommand(pactflow::build_cli())
//         .subcommand(
//             Command::new("generate")
//                 .about("Generate completion files for your shell")
//                 .arg(
//                     Arg::new("generator")
//                         .long("generate")
//                         .help_heading("The shell to generate completions for")
//                         .value_parser(PossibleValuesParser::new(&[Shell::Bash, Shell::Fish, Shell::Zsh]))
//                         .required(true)
//                         )
//         )
// }
// pub fn build_cli() -> Command {
//     Command::new("compl")
//         .about("Tests completions")
//         .arg(Arg::new("file")
//             .help("some input file"))
//         .subcommand(Command::new("test")
//             .about("tests things")
//             .arg(Arg::new("case")
//                 .long("case")
//                 .action(ArgAction::Set)
//                 .help("the case to test")))
// }
pub fn main() {
    // let cmd = build_cli();
    let _m = cli::build_cli().get_matches();

    match _m.subcommand() {
        Some(("subcommand1", _)) => {
            // Handle subcommand1 case
        }
        Some(("completions", args)) => {
            // Handle subcommand2 case
            let mut cmd = cli::build_cli();
            // print!("completions");
            print!("{:?}", args);
            let shell: String = args.get_one::<String>("shell").expect("a shell is required").to_string();
            let out_dir: String = args.get_one::<String>("dir").expect("a directory is expected").to_string();
            let shell_enum = Shell::from_str(&shell).unwrap();
            let _ = generate_to(shell_enum, &mut cmd, "pact_cli".to_string(), out_dir);
        }
        // Add arms to cover all cases for argmatches
        _ => {
            // Handle default case
        }
    }
    
    
    // generate(Shell::Bash, &cmd, "pact-cli".to_string(), &mut io::stdout());
}


// /// Pact CLI
// struct Arguments {
//     #[clap(subcommand)]
//     cmd: SubCommand
// }

// #[derive(Subcommand, Debug)]
// enum SubCommand {
//     // Pact
//     PactBroker(pact_broker::PactBrokerArguments),
//     // PactFlow
//     Pactflow(pactflow::PactflowArguments),
//     Generate,
// }

// fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
//     generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
// }


// // #[derive(Parser, Debug)]
// // #[clap(author = "PactFlow", version, about)]
// // pub(crate) struct GenerateArguments {
// //     #[clap(subcommand)]
// //    pub cmd: GenerateArgumentsSubCommand,
// // }

// // #[derive(Subcommand, Debug)]
// // pub enum GenerateArgumentsSubCommand {
// // Generate(GenerateArgumentsSubCommandGenerate),
// // }

// // pub struct GenerateArgumentsSubCommandGenerate {
// //     #[clap(
// //         // short = 'b',
// //         long = "content-file",
// //         required = true,
// //         num_args = 1,
// //         number_of_values = 1,
// //         value_parser = clap::builder::NonEmptyStringValueParser::new(),
// //         help = "Provider specification to publish"
// //     )]
// //     content_file: Option<String>,
// // }

// fn main() {
//     let args = Arguments::parse();
//     match args.cmd {
//         SubCommand::PactBroker(_args) => {
//             match _args.cmd {
//                 pact_broker::PactBrokerSubCommand::Publish(args) => {
//                     let result = pact_broker::publish_pact::main(args);
//                     print!("{:?}", result);
//             }}
//         }
//         SubCommand::Pactflow(_args) => {
//             match _args.cmd {
//                 pactflow::PactflowSubCommand::PublishProviderContract(args) => {
//                     let result = pactflow::publish_provider_contract::main(args);
//                     print!("{:?}", result);
//             }}
//         },
//         SubCommand::Generate => {
//             let mut cmdr = args.cmd;

//             // match _args.cmd {
//             //     Generate(GenerateArgumentsSubCommandGenerate) => {
//             //         let result = pactflow::publish_provider_contract::main(args);
//             //         print!("{:?}", result);
//             // }}

//             eprintln!("Generating completion file for");
//             let generator = Shell::Bash;
//             let 
//             print_completions(generator, &mut cmdr);
//         }
//     }
// }
