use clap::Args;

#[derive(Debug, Args)]
pub struct App2Args {
    #[clap(required = true, forbid_empty_values = true)]
    name: String,
}

pub fn subcommand2(args:App2Args) -> Result<String, String> {
    let name = args.name;
    Ok(name.to_string())
}