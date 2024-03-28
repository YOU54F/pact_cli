use crate::pact_broker::main::{HALClient, Link, PactBrokerError};
use comfy_table::{presets::UTF8_FULL, Table};
use maplit::hashmap;
use serde_json::Value;

use super::{
    types::{BrokerDetails, OutputType},
    utils::{follow_broker_relation, generate_table, get_broker_relation},
};

pub fn list_latest_pact_versions(
    broker_details: &BrokerDetails,
    output_type: OutputType,
    verbose: bool,
) -> Result<String, PactBrokerError> {
    // setup client with broker url and credentials
    let broker_url = &broker_details.url;
    let auth = &broker_details.auth;
    let res = tokio::runtime::Runtime::new().unwrap().block_on(async {
        // query pact broker index and get hal relation link
        let hal_client: HALClient = HALClient::with_url(broker_url, auth.clone());
        let pb_latest_pact_versions_href_path = get_broker_relation(
            hal_client.clone(),
            "pb:latest-pact-versions".to_string(),
            broker_url.to_string(),
        )
        .await;

        match pb_latest_pact_versions_href_path {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        // query the hal relation link to get the latest pact versions
        let res = follow_broker_relation(
            hal_client.clone(),
            "pb:latest-pact-versions".to_string(),
            pb_latest_pact_versions_href_path.unwrap(),
        )
        .await;
        match res {
            Ok(result) => match output_type {
                OutputType::Json => {
                    let json: String = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                    return Ok(json);
                }
                OutputType::Table => {
                    let table = generate_table(
                        &result,
                        vec!["CONSUMER", "CONSUMER_VERSION", "PROVIDER", "CREATED_AT"],
                        vec![
                            vec!["_embedded", "consumer", "name"],
                            vec!["_embedded", "consumer", "_embedded", "version", "number"],
                            vec!["_embedded", "provider", "name"],
                            vec!["createdAt"],
                        ],
                    );
                    println!("{table}");
                    return Ok(table.to_string());
                }

                OutputType::Text => {
                    let text = result.to_string();
                    println!("{:?}", text);
                    return Ok(text);
                }
                OutputType::Pretty => {
                    let json: String = serde_json::to_string(&result).unwrap();
                    println!("{}", json);
                    return Ok(json);
                }
            },
            Err(err) => Err(err),
        }
    });
    match res {
        Ok(result) => {
            return Ok(result);
        }
        Err(err) => {
            return Err(err);
        }
    }
}
