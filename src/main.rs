use app1::App1Args;
use app2::App2Args;
use clap::{Parser, Subcommand};
mod app1;
mod app2;
#[derive(Parser, Debug)]
#[clap(author = "Author Name", version, about)]
/// A Very simple App with multiple subcommands
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Run app 1
    App1(App1Args),
    /// Run app 1
    App2(App2Args),
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::App1(args) => match app1::subcommand1(args) {
            Ok(c) => println!("you provided {} to app1", c),
            Err(e) => {
                eprintln!("error in processing : {}", e);
                std::process::exit(1)
            }
        },
        SubCommand::App2(args) => match app2::subcommand2(args) {
            Ok(c) => println!("you provided {} to app2", c),
            Err(e) => {
                eprintln!("error in processing : {}", e);
                std::process::exit(1)
            }
        },
    }
}
