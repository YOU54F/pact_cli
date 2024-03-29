// use std::collections::HashMap;
mod cli;
use crate::cli::pact_mock_server_cli;
use crate::cli::pact_stub_server_cli;
use crate::cli::pact_verifier_cli;
use crate::cli::{pact_broker, pact_plugin_cli};
use clap::error::ErrorKind;
use clap::ArgMatches;
use clap_complete::{generate_to, Shell};
use cli::cli_extension;
use std::process::Command;
use std::str::FromStr;

pub fn main() {
    let app = cli::build_cli();
    let matches = app.clone().try_get_matches();

    match matches {
        Ok(results) => {
            match results.subcommand() {
                Some(("pact-broker", args)) => cli::pact_broker_client::run(args),
                Some(("pactflow", args)) => cli::pactflow_client::run(args),
                Some(("completions", args)) => generate_completions(args),
                Some(("standalone", args)) => cli::pact_broker_standalone::run(args),
                Some(("examples", args)) => process_examples_command(args),
                Some(("project", args)) => process_project_command(args),
                Some(("docker", args)) => cli::pact_broker_docker::run(args),
                Some(("plugin", args)) => process_plugin_command(args),
                Some(("mock", args)) => process_mock_command(args),
                Some(("stub", args)) => process_stub_command(args),
                Some(("verifier", args)) => process_verifier_command(args),
                Some(("extension", args)) => cli_extension::main(args),
                _ => cli::build_cli().print_help().unwrap(),
            }
        }
        Err(ref err) => match err.kind() {
            ErrorKind::DisplayHelp => {
                let _ = err.print();
            }
            ErrorKind::DisplayVersion => {
                let error_message = err.render().to_string();
                let mock_server_match = "pact_cli-mock \n".to_string();
                let verifier_match = "pact_cli-verifier \n".to_string();
                let stub_server_match = "pact_cli-stub \n".to_string();
                if verifier_match == error_message {
                    pact_verifier_cli::main::print_version(&verifier_match);
                    println!();
                } else if mock_server_match == error_message {
                    pact_mock_server_cli::main::print_version();
                    println!();
                } else if stub_server_match == error_message {
                    pact_stub_server_cli::main::print_version();
                    println!();
                }
            }
            _ => err.exit(),
        },
    }
}

fn process_plugin_command(args: &ArgMatches) {
    let res = pact_plugin_cli::main::run(args);
    match res {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            print!("Error: {:?}", e);
            std::process::exit(1);
        }
        _ => {
            std::process::exit(1);
        }
    }
}

fn process_mock_command(args: &ArgMatches) {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let res = pact_mock_server_cli::main::handle_matches(args).await;
        match res {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => {
                std::process::exit(e);
            }
            _ => {
                std::process::exit(1);
            }
        }
    });
}

fn process_stub_command(args: &ArgMatches) {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let res = pact_stub_server_cli::main::handle_matches(args).await;
        match res {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(3);
            }
            _ => {
                std::process::exit(1);
            }
        }
    });
}

fn process_verifier_command(args: &ArgMatches) {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let res = pact_verifier_cli::main::handle_matches(args).await;
        match res {
            Ok(_) => {
                std::process::exit(0);
            }
            Err(e) => {
                std::process::exit(e);
            }
        }
    });
}

fn generate_completions(args: &ArgMatches) {
    let shell = args
        .get_one::<String>("shell")
        .expect("a shell is required");
    let out_dir = args
        .get_one::<String>("dir")
        .expect("a directory is expected")
        .to_string();
    let mut cmd = cli::build_cli();
    let shell_enum = Shell::from_str(&shell).unwrap();
    let _ = generate_to(shell_enum, &mut cmd, "pact_cli".to_string(), &out_dir);
    println!(
        "ℹ️  {} shell completions for pact_cli written to {}",
        &shell_enum, &out_dir
    );
}

fn process_examples_command(args: &ArgMatches) {
    let project_type = args.get_one::<String>("type").unwrap().as_str();
    let project = &args
        .get_one::<String>("project")
        .map(|project| project.to_string());
    let download_all = args.get_flag("all");

    match project_type {
        "bdct" => {
            let projects = vec![
                "example-bi-directional-consumer-cypress",
                "example-bi-directional-provider-postman",
                "example-bi-directional-consumer-msw",
                "example-bi-directional-provider-dredd",
                "example-bi-directional-provider-restassured",
                "example-bi-directional-consumer-wiremock",
                "example-bi-directional-consumer-nock",
                "example-bi-directional-consumer-mountebank",
                "example-bi-directional-consumer-dotnet",
                "example-bi-directional-provider-dotnet",
            ];

            if download_all {
                for project in projects {
                    download_project(project);
                }
            } else if let Some(project) = project {
                download_project(project);
            } else {
                println!("Please specify a project to download");
                for project in projects {
                    println!("{}", project);
                }
            }
        }
        "cdct" => {
            let projects = vec![
                "example-siren",
                "example-provider",
                "example-consumer",
                "example-consumer-js-kafka",
                "example-consumer-cypress",
                "example-consumer-python",
                "example-consumer-golang",
                "example-consumer-java-kafka",
                "example-consumer-java-junit",
                "example-consumer-java-soap",
                "example-consumer-dotnet",
                "example-provider-golang",
                "example-provider-springboot",
                "example-provider-java-soap",
                "example-provider-java-kafka",
                "example-consumer-js-sns",
                "example-provider-js-sns",
                "example-provider-python",
                "example-consumer-webhookless",
                "example-provider-dotnet",
                "pactflow-jsonschema-example",
                "provider-driven-example",
                "injected-provider-states-example",
            ];

            if download_all {
                for project in projects {
                    download_project(project);
                }
            } else if let Some(project) = project {
                download_project(project);
            } else {
                println!("Please specify a project to download");
                for project in projects {
                    println!("{}", project);
                }
            }
        }
        "workshops" => {
            let projects = vec![
                "pact-workshop-js",
                "pact-workshop-jvm-spring",
                "pact-workshop-dotnet-core-v1",
                "pact-workshop-Maven-Springboot-JUnit5",
                "pact-workshop-go",
            ];
            let org = "pact-foundation";

            if download_all {
                for project in projects {
                    download_project_with_org(org, project);
                }
            } else if let Some(project) = project {
                download_project_with_org(org, project);
            } else {
                println!("Please specify a project to download");
                for project in projects {
                    println!("{}", project);
                }
            }
        }
        _ => {
            println!("Sorry, you'll need to specify a valid option (bdct, cdct, workshops)");
        }
    }

    fn download_project(project: &str) {
        println!("Downloading {}", project);
        // Implement the logic to download the project here
        println!("Downloaded {}", project);
        println!("Unimplemented");
        std::process::exit(1);
    }

    fn download_project_with_org(org: &str, project: &str) {
        println!("Downloading project {}", project);
        // Implement the logic to download the project with the specified organization here
        println!("Downloaded project {}", project);

        println!("Unimplemented");
        std::process::exit(1);
    }
}
fn process_project_command(args: &ArgMatches) {
    match args.subcommand() {
        Some(("install", args)) => {
            let language = args.get_one::<String>("language").unwrap().as_str();
            match language {
                "js" => {
                    println!("To install Pact-JS, run the following command:");
                    println!("`npm install @pact-foundation/pact`");
                }
                "golang" => {
                    println!("To install Pact-Go, run the following command:");
                    println!(
                        "`go get github.com/pact-foundation/pact-go/v2@2.x.x`"
                    );
                    println!("# NOTE: If using Go 1.19 or later, you need to run go install instead");
                    println!(
                        "# go install github.com/pact-foundation/pact-go/v2@2.x.x"
                    );
                    println!("# download and install the required libraries. The pact-go will be installed into $GOPATH/bin, which is $HOME/go/bin by default.");
                    println!("pact-go -l DEBUG install");
                    println!("# 🚀 now write some tests!");
                }
                "ruby" => {
                    println!("To install Pact-Ruby, run the following command:");
                    println!("Add this line to your application's Gemfile:");
                    println!("gem 'pact'");
                    println!("# gem 'pact-consumer-minitest' for minitest");
                    println!("And then execute:");
                    println!("$ bundle");
                    println!("Or install it yourself as:");
                    println!("$ gem install pact");
                }
                "python" => {
                    println!("To install Pact-Python, run the following command:");
                    println!("`pip install pact-python`");
                }
                "java" => {
                    println!("To install Pact-JVM, add the following dependency to your build file:");
                    println!("`testImplementation 'au.com.dius.pact.consumer:junit5:4.6.5'`");
                    println!("`testImplementation 'au.com.dius.pact.provider:junit5:4.6.5'`");
                }
                ".net" => {
                    println!("To install Pact-.NET, add the following package to your project:");
                    println!("`dotnet add package PactNet --version 4.5.0`");
                }
                "rust" => {
                    println!("To install Pact-Rust, add the following dependency to your Cargo.toml file:");
                    println!("`pact_consumer = \"0.0.1\"`");
                    println!("`pact_verifier = \"0.0.1\"`");
                    println!("`pact_models = \"0.0.1\"`");
                    println!("`pact_matching = \"0.0.1\"`");
                }
                "php" => {
                    println!("To install Pact-PHP, add the following dependency to your composer.json file:");
                    println!("`\"pact-foundation/pact-php\": \"^9.0\"`");
                    println!("To try out Pact-PHP build with the pact rust core:");
                    println!("`\"pact-foundation/pact-php\": \"^10.0.0-alpha6\"`");
                }
                _ => {
                    println!("⚠️  Invalid option provided");
                    // Ok(());
                }
            }
        }
        Some(("new", args)) => {
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("link", args)) => {
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("issue", args)) => {
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("docs", args)) => {
            println!("Unimplemented");
            std::process::exit(1);
        }
        _ => {
            println!("⚠️  No option provided, try running project --help");
        }
    }
}