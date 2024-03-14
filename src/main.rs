// use std::collections::HashMap;
mod cli;
use clap_complete::{generate_to, Shell};
use pact_broker::{pact_broker::{HALClient, Link, PactBrokerError, links_from_json}};
use serde_json::Value;
// mod pact_broker::pact_broker::HALClient::Link;
use std::str::FromStr;
mod pact_broker;
use pact_models::http_utils::HttpAuth;
use maplit::hashmap;
// use futures::executor::block_on;
use futures::stream::*;
use pact_models::pact::{load_pact_from_json, Pact};
use tabled::{builder::Builder, settings::Style, Table};

#[tokio::main]
pub async fn main() {
    let _m = cli::build_cli().get_matches();

    match _m.subcommand() {
        Some(("pact-broker", args)) => {
            match args.subcommand() {
                Some(("publish", args)) => {
                    print!("{:?}", args);
                    // // Ok(());
                }
                Some(("list-latest-pact-versions", args)) => {
                    // Handle list-latest-pact-versions command
                    let broker_url: String = args.get_one::<String>("broker-base-url").expect("url is required").to_string();
                    let token = args.try_get_one::<String>("broker-token");
                    let username = args.try_get_one::<String>("broker-username");
                    let password = args.try_get_one::<String>("broker-password");
                    let output = args.try_get_one::<String>("output");

                    let auth;
                    match token {
                        Ok(Some(token)) => {
                            auth = HttpAuth::Token(token.to_string());
                        }
                        Ok(None) => {
                            match username {
                                Ok(Some(username)) => {
                                    // auth = HttpAuth::User(username.clone().expect("username required"), None);

                                    match password {
                                        Ok(Some(password)) => {
                                            auth = HttpAuth::User(username.to_string(), Some(password.to_string()));
                                        }
                                        Ok(None) => {
                                            auth = HttpAuth::User(username.to_string(), None);
                                        }
                                        Err(_) => todo!()
                                    }
                                }
                                Ok(None) => {
                                    auth = HttpAuth::None;
                                }
                                Err(_) => todo!()
                            }
                            // auth = HttpAuth::None;
                        }
                        Err(_) => todo!()
                    }

                    let mut hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth));
                    // TODO - follow link without href
                    let link = Link { name: "pb:latest-pact-versions".to_string(), href: Some("/pacts/latest".to_string()), templated: false, title: None };
                    // let link = Link { name: "pb:latest-pact-versions".to_string(), href: Some("/pacts/latest".to_string()), templated: false, title: None };
                    let template_values = hashmap!{};
                    let res: Result<Value, PactBrokerError> = hal_client.fetch_url(&link, &template_values).await;
                    match output {
                        Ok(Some(output)) => {
                            if output == "json" {
                                let json: String = serde_json::to_string(&res.clone().unwrap()).unwrap();
                                println!("{}", json);
                            } else if output == "table" {
                                if let Ok(res) = res {
                                    let mut builder = Builder::default();
                                    builder.push_record(["CONSUMER", "CONSUMER_VERSION", "PROVIDER", "CREATED_AT"]);

 
                                    if let Some(items) = res.get("pacts").unwrap().as_array() {
                                        for item in items {
                                            let consumer = &item["_embedded"]["consumer"]["name"].to_string();
                                            let consumer_version = &item["_embedded"]["consumer"]["_embedded"]["version"]["number"].to_string();
                                            let provider = &item["_embedded"]["provider"]["name"].to_string();
                                            let created_at = &item["createdAt"].to_string();
                                            builder.push_record([consumer, consumer_version, provider, created_at]);
                                        }
                                    }
                                    let mut table = builder.build();
                                    table.with(Style::rounded());
                                    
                                    println!("{:#}", table);
                                }
                            }
                        }
                        Ok(None) => {
                            println!("{:?}", res.clone());
                        }
                        Err(_) => todo!()
                    }



    
                }
                Some(("create-environment", args)) => {
                    // Handle create-environment command
                    // Ok(());
                }
                Some(("update-environment", args)) => {
                    // Handle update-environment command
                    // Ok(());
                }
                Some(("describe-environment", args)) => {
                    // Handle describe-environment command
                    // Ok(());
                }
                Some(("delete-environment", args)) => {
                    // Handle delete-environment command
                    // Ok(());
                }
                Some(("list-environments", args)) => {
                    // Handle list-environments command
                    // Ok(());
                }
                Some(("record-deployment", args)) => {
                    // Handle record-deployment command
                    // Ok(());
                }
                Some(("record-undeployment", args)) => {
                    // Handle record-undeployment command
                    // Ok(());
                }
                Some(("record-release", args)) => {
                    // Handle record-release command
                    // Ok(());
                }
                Some(("record-support-ended", args)) => {
                    // Handle record-support-ended command
                    // Ok(());
                }
                Some(("can-i-deploy", args)) => {
                    // Handle can-i-deploy command
                    // Ok(());
                }
                Some(("can-i-merge", args)) => {
                    // Handle can-i-merge command
                    // Ok(());
                }
                Some(("create-or-update-pacticipant", args)) => {
                    // Handle create-or-update-pacticipant command
                    // Ok(());
                }
                Some(("create-webhook", args)) => {
                    // Handle create-webhook command
                    // Ok(());
                }
                Some(("create-or-update-webhook", args)) => {
                    // Handle create-or-update-webhook command
                    // Ok(());
                }
                Some(("test-webhook", args)) => {
                    // Handle test-webhook command
                
                    // Ok(());
                }
                _ => {
                    println!("⚠️  No option provided, try running pact-broker --help");

                    // Ok(());
                }
            }
        }
        Some(("pactflow", args)) => {
            match args.subcommand() {
                Some(("publish-provider-contract", args)) => {
                    print!("{:?}", args);

                    // Ok(());
                }
                _ => {
                    println!("⚠️  No option provided, try running pactflow --help");

                    // Ok(());
                }
            }


        }
        Some(("completions", args)) => {
            let mut cmd = cli::build_cli();
            let shell: String = args.get_one::<String>("shell").expect("a shell is required").to_string();
            let out_dir: String = args.get_one::<String>("dir").expect("a directory is expected").to_string();
            let shell_enum = Shell::from_str(&shell).unwrap();
            let _ = generate_to(shell_enum, &mut cmd, "pact_cli".to_string(), &out_dir);
            print!("ℹ️  {} shell completions for pact_cli written to {}", &shell_enum, &out_dir);

            // Ok(());
        }
        _ => {
            cli::build_cli().print_help().unwrap();

            // Ok(());
        }
    }
}

