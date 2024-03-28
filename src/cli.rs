use clap::{Arg, Command};

pub mod pact_broker;
pub mod pact_broker_client;
pub mod pact_broker_docker;
pub mod pact_broker_standalone;
pub mod pact_mock_server_cli;
pub mod pact_plugin_cli;
pub mod pact_stub_server_cli;
pub mod pact_verifier_cli;
pub mod pactflow_client;
mod utils;

pub fn build_cli() -> Command {
    let app = Command::new("pact_cli")
        .about("A pact cli tool")
        .subcommand(pact_broker_client::add_pact_broker_client_command())
        .subcommand(pactflow_client::add_pactflow_client_command())
        .subcommand(add_completions_subcommand())
        .subcommand(pact_broker_docker::add_docker_broker_subcommand())
        .subcommand(add_examples_subcommand())
        .subcommand(add_project_subcommand())
        .subcommand(pact_broker_standalone::add_standalone_broker_subcommand())
        .subcommand(pact_plugin_cli::main::add_plugin_cli_subcommand().arg_required_else_help(true))
        .subcommand(pact_mock_server_cli::main::setup_args())
        .subcommand(pact_stub_server_cli::main::build_args())
        .subcommand(
            pact_verifier_cli::main::build_args()
                .arg_required_else_help(true)
                .disable_version_flag(true),
        );
    // Continue adding other subcommands as needed
    // ...
    app
}

pub fn add_output_arguments(
    value_parser_args: Vec<&'static str>,
    default_value: &'static str,
) -> Vec<Arg> {
    vec![Arg::new("output")
        .short('o')
        .long("output")
        .value_name("OUTPUT")
        .value_parser(clap::builder::PossibleValuesParser::new(&value_parser_args))
        .default_value(default_value) // Fix: Remove the borrow operator
        .value_name("OUTPUT")
        .help(format!("Value must be one of {:?}", value_parser_args))]
}

pub fn add_verbose_arguments() -> Vec<Arg> {
    vec![Arg::new("verbose")
        .short('v')
        .long("verbose")
        .num_args(0)
        .help("Verbose output.")]
}

fn add_completions_subcommand() -> Command {
    Command::new("completions") 
    .about("Generates completion scripts for your shell")
    .arg(Arg::new("shell")
        .value_name("SHELL")
        .required(true)
        .value_parser(clap::builder::PossibleValuesParser::new(&["bash", "fish", "zsh", "powershell", "elvish"]))
        .help("The shell to generate the script for"))
    .arg(Arg::new("dir")
        .short('d')
        .long("dir")
        .value_name("DIRECTORY")
        .required(false)
        .default_value(".")
        .num_args(1)
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("The directory to write the shell completions to, default is the current directory"))
}

fn add_examples_subcommand() -> Command {
    Command::new("examples")
        .about("download example projects")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .num_args(1)
                .value_parser(clap::builder::PossibleValuesParser::new(&[
                    "bdct",
                    "cdct",
                    "workshops",
                ]))
                .required(true)
                .help("Specify the project type (bdct, cdct, workshops)"),
        )
        .arg(
            Arg::new("project")
                .short('p')
                .long("project")
                .num_args(1)
                .help("Specify the project to download"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Download all projects")
                .action(clap::ArgAction::SetTrue),
        )
}

fn add_project_subcommand() -> Command {
    Command::new("project")
        .about("Pact project actions for setting up and managing pact projects")
        .subcommand(
            Command::new("install").about("install pact").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language to install pact for"),
            ),
        )
        .subcommand(
            Command::new("new").about("create new pact project").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for the new pact project"),
            ),
        )
        .subcommand(
            Command::new("link").about("link pact project").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language of the pact project to link"),
            ),
        )
        .subcommand(
            Command::new("issue").about("create pact issue").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for creating the pact issue"),
            ),
        )
        .subcommand(
            Command::new("docs").about("open pact documentation").arg(
                Arg::new("language")
                    .short('l')
                    .long("language")
                    .num_args(1)
                    .value_parser(clap::builder::PossibleValuesParser::new(&[
                        "js", "golang", "ruby", "python", "java", ".net", "rust", "php",
                    ]))
                    .required(true)
                    .help("Specify the language for opening the pact documentation"),
            ),
        )
}
