use std::env;
use clap::{value_parser, ArgMatches, Command};
use std::process::{Command as Cmd,exit};
use std::io::{self, Write};

pub fn add_cli_extensions_subcommand() -> Command {
    Command::new("extension")
    .about("Pact CLI extensions are repositories that provide additional gh commands")
    .allow_external_subcommands(true)
    .external_subcommand_value_parser(value_parser!(String))
    // .subcommand(Command::new("browse"))
    // .subcommand(Command::new("create"))
    // .subcommand(Command::new("exec"))
    // .subcommand(Command::new("install"))
    // .subcommand(Command::new("list"))
    // .subcommand(Command::new("remove"))
    // .subcommand(Command::new("search"))
    // .subcommand(Command::new("upgrade"))   
}

pub fn main(_arg: &ArgMatches) {
    // print!("{:?}", arg.subcommand().unwrap());
    let mut args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // remove the first three args assuming they are the binary name, the extension command and the extension name
    // "target/debug/pact_cli", "extension", "--"
    args.drain(0..3);
    println!("{:?}", args);

    if args.len() < 1 {
        eprintln!("No extension name provided");
        exit(1);
    }

    let extension_name = &args[0];
    let extensions_home = env::var("PACT_CLI_EXTENSIONS_HOME").unwrap_or(".pact/extensions".to_string());
    let extension_executable = format!("pact_cli-{}", extension_name);
    let extension_executable = if extensions_home.ends_with("/") {
        format!("{}{}/./{}", extensions_home, extension_executable, extension_executable)
    } else {
        format!("{}/{}/{}", extensions_home, extension_executable, extension_executable)
    };

    println!("Extension executable: {}", extension_executable);
    let mut command = Cmd::new(&extension_executable);
    command.args(&args[1..]);
    command.stdout(io::stdout());
    command.stderr(io::stderr());

    let status = command.status().expect("Failed to execute extension command");

    if !status.success() {
        // eprintln!("Extension command failed with status: {}", status);
        exit(status.code().unwrap_or(1));
    }
}
