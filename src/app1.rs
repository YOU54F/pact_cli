use clap::Args;

#[derive(Debug, Args)]
pub struct App1Args {
    #[clap(forbid_empty_values = true)]
    name: Option<String>,
}

pub fn subcommand1(args:App1Args) -> Result<String, String> {
    let name = args.name;
    match name {
        Some(name) => Ok(name.to_string()),
        None => Err("No name provided".to_string()),
    }
}