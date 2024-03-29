use super::utils;
use crate::cli::pact_broker::main::list_latest_pact_versions::list_latest_pact_versions;
use crate::cli::pact_broker::main::pact_publish;
use crate::cli::pact_broker::main::types::{BrokerDetails, OutputType};
use crate::cli::pact_broker::main::utils::{get_auth, get_broker_url, handle_error};
use crate::cli::pact_broker::main::{HALClient, PactBrokerError};
use ansi_term::Colour;
use clap::{Arg, ArgGroup, ArgMatches, Command};
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
pub fn add_pact_broker_client_command() -> Command {
    Command::new("pact-broker")
        .args(crate::cli::add_output_arguments(
            ["json", "text", "table", "pretty"].to_vec(),
            "text",
        ))
        .subcommand(add_publish_pacts_subcommand())
        .subcommand(add_list_latest_pact_versions_subcommand())
        .subcommand(add_create_environment_subcommand())
        .subcommand(add_update_environment_subcommand())
        .subcommand(add_delete_environment_subcommand())
        .subcommand(add_describe_environment_subcommand())
        .subcommand(add_list_environments_subcommand())
        .subcommand(add_record_deployment_subcommand())
        .subcommand(add_record_undeployment_subcommand())
        .subcommand(add_record_release_subcommand())
        .subcommand(add_record_support_ended_subcommand())
        .subcommand(add_can_i_deploy_subcommand())
        .subcommand(add_can_i_merge_subcommand())
        .subcommand(add_create_or_update_pacticipant_subcommand())
        .subcommand(add_describe_pacticipant_subcommand())
        .subcommand(add_list_pacticipants_subcommand())
        .subcommand(add_create_webhook_subcommand())
        .subcommand(add_create_or_update_webhook_subcommand())
        .subcommand(add_test_webhook_subcommand())
        .subcommand(add_delete_branch_subcommand())
        .subcommand(add_create_version_tag_subcommand())
        .subcommand(add_describe_version_subcommand())
        .subcommand(add_create_or_update_version_subcommand())
        .subcommand(add_generate_uuid_subcommand())
}

pub fn add_broker_auth_arguments() -> Vec<Arg> {
    vec![
        Arg::new("broker-base-url")
            .short('b')
            .long("broker-base-url")
            .num_args(1)
            .help("The base URL of the Pact Broker")
            .required(true)
            .value_name("PACT_BROKER_BASE_URL")
            .env("PACT_BROKER_BASE_URL"),
        Arg::new("broker-username")
            .short('u')
            .long("broker-username")
            .num_args(1)
            .help("Pact Broker basic auth username")
            .value_name("PACT_BROKER_USERNAME")
            .env("PACT_BROKER_USERNAME"),
        Arg::new("broker-password")
            .short('p')
            .long("broker-password")
            .num_args(1)
            .help("Pact Broker basic auth password")
            .value_name("PACT_BROKER_PASSWORD")
            .env("PACT_BROKER_PASSWORD"),
        Arg::new("broker-token")
            .short('k')
            .long("broker-token")
            .num_args(1)
            .help("Pact Broker bearer token")
            .value_name("PACT_BROKER_TOKEN")
            .env("PACT_BROKER_TOKEN"),
    ]
}

fn add_publish_pacts_subcommand() -> Command {
    Command::new("publish")
    .args(add_broker_auth_arguments())
    .about("Publishes pacts to the Pact Broker")
    // .arg(Arg::new("PACT_DIRS_OR_FILES")
    //     .num_args(0..=1)
    //     .required(true)
    //     .help("Pact directories or files"))

    .arg(Arg::new("url")
    // .short('b')
    .long("url")
    .num_args(1)
    .help("The url of the pact file"))
.arg(Arg::new("username")
    // .short('u')
    .long("username")
    .num_args(1)
    .help("username for pact url auth"))
.arg(Arg::new("password")
    // .short('p')
    .long("password")
    .num_args(1)
    .help("password for pact url auth"))
.arg(Arg::new("token")
    // .short('k')
    .long("token")
    .num_args(1)
    .help("bearer token for pact url"))
.arg(Arg::new("file")
.short('f')
.long("file")
.num_args(0..)
// .num_args(0..=1)
.value_parser(clap::builder::NonEmptyStringValueParser::new())
.help("Pact file to publish (can be repeated)"))
.arg(Arg::new("dir")
.short('d')
.long("dir")
.num_args(0..)
// .num_args(0..=1)
.value_parser(clap::builder::NonEmptyStringValueParser::new())
.help("Directory of pact files to publish (can be repeated)"))
.arg(Arg::new("glob")
.short('g')
.long("glob")
// .action(ArgAction::Append)
// .value_delimiter(' ')
.num_args(0..)
.value_parser(clap::builder::NonEmptyStringValueParser::new())
.value_parser(|arg0: &str| utils::glob_value(arg0.to_string()))
.help("Glob pattern to match pact files to publish (can be repeated)")
.long_help("
Glob pattern to match pact files to publish

?      matches any single character.
*      matches any (possibly empty) sequence of characters.
**     matches the current directory and arbitrary subdirectories. This sequence must form
         a single path component, so both **a and b** are invalid and will result in an
         error. A sequence of more than two consecutive * characters is also invalid.
[...]  matches any character inside the brackets. Character sequences can also specify
         ranges of characters, as ordered by Unicode, so e.g. [0-9] specifies any character
         between 0 and 9 inclusive. An unclosed bracket is invalid.
[!...] is the negation of [...], i.e. it matches any characters not in the brackets.

The metacharacters ?, *, [, ] can be matched by using brackets (e.g. [?]). When a ]
occurs immediately following [ or [! then it is interpreted as being part of, rather
then ending, the character set, so ] and NOT ] can be matched by []] and [!]] respectively.
The - character can be specified inside a character sequence pattern by placing it at
the start or the end, e.g. [abc-].

See https://docs.rs/glob/0.3.0/glob/struct.Pattern.html"))
   .group(ArgGroup::new("publish")
.args(["glob", "file", "dir"].to_vec())
.multiple(true)
.required(true))
.arg(Arg::new("validate")
.long("validate")
// .short('v')
.num_args(0)
.action(clap::ArgAction::SetTrue)
.help("Validate the Pact files before publishing."))
.arg(Arg::new("strict")
.long("strict")
// .short('v')
.num_args(0)
.action(clap::ArgAction::SetTrue)
.help("Require strict validation."))
.arg(Arg::new("consumer-app-version")
   .short('a')
   .long("consumer-app-version")
   .value_parser(clap::builder::NonEmptyStringValueParser::new())
   .help("The consumer application version"))
.arg(Arg::new("branch")
   // .short('h')
   .long("branch")
   .value_parser(clap::builder::NonEmptyStringValueParser::new())
   .help("Repository branch of the consumer version"))
.arg(Arg::new("auto-detect-version-properties")
   .short('r')
   .long("auto-detect-version-properties")
   .num_args(0)
   .action(clap::ArgAction::SetTrue)
   .help("Automatically detect the repository commit, branch and build URL from known CI environment variables or git CLI. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps."))
.arg(Arg::new("tag")
   .short('t')
   .long("tag")
   .value_delimiter(',')
   .num_args(0..)
   .value_parser(clap::builder::NonEmptyStringValueParser::new())
   .help("Tag name for consumer version. Can be specified multiple times (delimiter ,)."))
.arg(Arg::new("tag-with-git-branch")
   // .short('g')
   .long("tag-with-git-branch")
   .num_args(0)
   .action(clap::ArgAction::SetTrue)
   .help("Tag consumer version with the name of the current git branch. Supports Buildkite, Circle CI, Travis CI, GitHub Actions, Jenkins, Hudson, AppVeyor, GitLab, CodeShip, Bitbucket and Azure DevOps."))
.arg(Arg::new("build-url")
   .long("build-url")
   .num_args(1)
   .help("The build URL that created the pact"))
.arg(Arg::new("merge")
   .long("merge")
   .num_args(0)
   .action(clap::ArgAction::SetTrue)
   .help("If a pact already exists for this consumer version and provider, merge the contents. Useful when running Pact tests concurrently on different build nodes."))
.args(crate::cli::add_output_arguments(["json", "text", "pretty"].to_vec(),"text"))
.args(crate::cli::add_verbose_arguments())
}

fn add_list_latest_pact_versions_subcommand() -> Command {
    Command::new("list-latest-pact-versions")
        .about("List the latest pact for each integration")
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
        .args(crate::cli::add_output_arguments(
            ["json", "table"].to_vec(),
            "table",
        ))
}
fn add_create_environment_subcommand() -> Command {
    Command::new("create-environment")
    .about("Create an environment resource in the Pact Broker to represent a real world deployment or release environment")
    .arg(Arg::new("name")
        .long("name")
        .value_name("NAME")
        .required(true)
        .help("The uniquely identifying name of the environment as used in deployment code"))
    .arg(Arg::new("display-name")
        .long("display-name")
        .value_name("DISPLAY_NAME")
        .help("The display name of the environment"))
    .arg(Arg::new("production")
        .long("production")
        .action(clap::ArgAction::SetTrue)
        .help("Whether or not this environment is a production environment. This is currently informational only."))
    .arg(Arg::new("contact-name")
        .long("contact-name")
        .value_name("CONTACT_NAME")
        .help("The name of the team/person responsible for this environment"))
    .arg(Arg::new("contact-email-address")
        .long("contact-email-address")
        .value_name("CONTACT_EMAIL_ADDRESS")
        .help("The email address of the team/person responsible for this environment"))
        .args(crate::cli::add_output_arguments(["json", "text", "id"].to_vec(), "text"))

.args(add_broker_auth_arguments())
.args(crate::cli::add_verbose_arguments())
}
fn add_update_environment_subcommand() -> Command {
    Command::new("update-environment")
    .about("Update an environment resource in the Pact Broker")
    .arg(Arg::new("uuid")
        .long("uuid")
        .value_name("UUID")
        .required(true)
        .help("The UUID of the environment to update"))
    .arg(Arg::new("name")
        .long("name")
        .value_name("NAME")
        .help("The uniquely identifying name of the environment as used in deployment code"))
    .arg(Arg::new("display-name")
        .long("display-name")
        .value_name("DISPLAY_NAME")
        .help("The display name of the environment"))
    .arg(Arg::new("production")
        .long("production")
        .action(clap::ArgAction::SetTrue)
        .help("Whether or not this environment is a production environment. This is currently informational only."))
    .arg(Arg::new("contact-name")
        .long("contact-name")
        .value_name("CONTACT_NAME")
        .help("The name of the team/person responsible for this environment"))
    .arg(Arg::new("contact-email-address")
        .long("contact-email-address")
        .value_name("CONTACT_EMAIL_ADDRESS")
        .help("The email address of the team/person responsible for this environment"))
        .args(crate::cli::add_output_arguments(["json", "text", "id"].to_vec(), "text"))
.args(add_broker_auth_arguments())
.args(crate::cli::add_verbose_arguments())
}
fn add_describe_environment_subcommand() -> Command {
    Command::new("describe-environment")
        .about("Describe an environment")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .required(true)
                .help("The UUID of the environment to describe"),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}
fn add_delete_environment_subcommand() -> Command {
    Command::new("delete-environment")
        .about("Delete an environment")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .required(true)
                .help("The UUID of the environment to delete"),
        )
        // .args(crate::cli::add_output_arguments(["json", "text"].to_vec(), "text"))
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}

fn add_list_environments_subcommand() -> Command {
    Command::new("list-environments")
        .about("List environments")
        .args(crate::cli::add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}
fn add_record_deployment_subcommand() -> Command {
    Command::new("record-deployment")
    .about("Record deployment of a pacticipant version to an environment")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the pacticipant that was deployed"))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The pacticipant version number that was deployed"))
    .arg(Arg::new("environment")
        .long("environment")
        .value_name("ENVIRONMENT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the environment that the pacticipant version was deployed to"))
    .arg(Arg::new("application-instance")
        .long("application-instance")
        .value_name("APPLICATION_INSTANCE")
        .alias("target")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("Optional. The application instance to which the deployment has occurred - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release"))
    .args(crate::cli::add_output_arguments(
        ["json", "text", "pretty"].to_vec(),
        "text",
    ))

.args(add_broker_auth_arguments())
.args(crate::cli::add_verbose_arguments())
}
fn add_record_undeployment_subcommand() -> Command {
    Command::new("record-undeployment")
    .about("Record undeployment of a pacticipant version from an environment")
    .long_about("Record undeployment of a pacticipant version from an environment.\n\nNote that use of this command is only required if you are permanently removing an application instance from an environment. It is not required if you are deploying over a previous version, as record-deployment will automatically mark the previously deployed version as undeployed for you. See https://docs.pact.io/go/record-undeployment for more information.")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the pacticipant that was undeployed"))
    .arg(Arg::new("environment")
        .long("environment")
       .value_name("ENVIRONMENT")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .required(true)
        .help("The name of the environment that the pacticipant version was undeployed from"))
    .arg(Arg::new("application-instance")
        .long("application-instance")
        .alias("target")
        .value_name("APPLICATION_INSTANCE")
        .value_parser(clap::builder::NonEmptyStringValueParser::new())
        .help("Optional. The application instance from which the application is being undeployed - a logical identifer required to differentiate deployments when there are multiple instances of the same application in an environment. This field was called 'target' in a beta release"))

    .args(add_broker_auth_arguments())
    .args(crate::cli::add_verbose_arguments())
    .args(crate::cli::add_output_arguments(
        ["json", "text", "pretty"].to_vec(),
        "text",
    ))
}

fn add_record_release_subcommand() -> Command {
    Command::new("record-release")
        .about("Record release of a pacticipant version to an environment.")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The name of the pacticipant that was released."),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number that was released."),
        )
        .arg(
            Arg::new("environment")
                .long("environment")
                .value_name("ENVIRONMENT")
                .required(true)
                .help("The name of the environment that the pacticipant version was released to."),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}
fn add_record_support_ended_subcommand() -> Command {
    Command::new("record-support-ended")
        .about("Record the end of support for a pacticipant version in an environment.")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The name of the pacticipant."),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number for which support is ended."),
        )
        .arg(
            Arg::new("environment")
                .long("environment")
                .value_name("ENVIRONMENT")
                .required(true)
                .help("The name of the environment in which the support is ended."),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text", "pretty"].to_vec(),
            "text",
        ))
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}
fn add_can_i_deploy_subcommand() -> Command {
    Command::new("can-i-deploy")
    .about("Check if a pacticipant can be deployed.")
    .long_about(
    r"
    Check if a pacticipant can be deployed.

    Description:
    Returns exit code 0 or 1, indicating whether or not the specified application (pacticipant) has a successful verification result with
    each of the application versions that are already deployed to a particular environment. Prints out the relevant pact/verification
    details, indicating any missing or failed verification results.
  
    The can-i-deploy tool was originally written to support specifying versions and dependencies using tags. This usage has now been
    superseded by first class support for environments, deployments and releases. For documentation on how to use can-i-deploy with tags,
    please see https://docs.pact.io/pact_broker/client_cli/can_i_deploy_usage_with_tags/
  
    Before `can-i-deploy` can be used, the relevant environment resources must first be created in the Pact Broker using the
    `create-environment` command. The 'test' and 'production' environments will have been seeded for you. You can check the existing
    environments by running `pact-broker list-environments`. See https://docs.pact.io/pact_broker/client_cli/readme#environments for more
    information.
  
    $ pact-broker create-environment --name 'uat' --display-name 'UAT' --no-production
  
    After an application is deployed or released, its deployment must be recorded using the `ecord-deployment` or `ecord-release`
    commands. See https://docs.pact.io/pact_broker/recording_deployments_and_releases/ for more information.
  
    $ pact-broker record-deployment --pacticipant Foo --version 173153ae0 --environment uat
  
    Before an application is deployed or released to an environment, the can-i-deploy command must be run to check that the application
    version is safe to deploy with the versions of each integrated application that are already in that environment.
  
    $ pact-broker can-i-deploy --pacticipant PACTICIPANT --version VERSION --to-environment ENVIRONMENT
  
    Example: can I deploy version 173153ae0 of application Foo to the test environment?
  
    $ pact-broker can-i-deploy --pacticipant Foo --version 173153ae0 --to-environment test
  
    Can-i-deploy can also be used to check if arbitrary versions have a successful verification. When asking 'Can I deploy this
    application version with the latest version from the main branch of another application' it functions as a 'can I merge' check.
  
    $ pact-broker can-i-deploy --pacticipant Foo 173153ae0 \\ --pacticipant Bar --latest main
  
    ##### Polling
  
    If the verification process takes a long time and there are results missing when the can-i-deploy command runs in your CI/CD pipeline,
    you can configure the command to poll and wait for the missing results to arrive. The arguments to specify are `--retry-while-unknown
    TIMES` and `--retry-interval SECONDS`, set to appropriate values for your pipeline.
    "
    )
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .num_args(0..=1)
        .help("The pacticipant name. Use once for each pacticipant being checked."))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version. Must be entered after the --pacticipant that it relates to."))
    .arg(Arg::new("ignore")
        .long("ignore")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .help("The pacticipant name to ignore. Use once for each pacticipant being ignored. A specific version can be ignored by also specifying a --version after the pacticipant name option. The environment variable PACT_BROKER_CAN_I_DEPLOY_IGNORE may also be used to specify a pacticipant name to ignore, with commas to separate multiple pacticipant names if necessary."))
    .arg(Arg::new("latest")
        .short('l')
        .long("latest")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .value_name("LATEST")
        .help("Use the latest pacticipant version. Optionally specify a TAG to use the latest version with the specified tag."))
    .arg(Arg::new("branch")
        .long("branch")
        .value_name("BRANCH")
        .help("The branch of the version for which you want to check the verification results."))
    .arg(Arg::new("main-branch")
        .long("main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["no-main-branch", "skip-main-branch"])
        .help("Use the latest version of the configured main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("no-main-branch")
        .long("no-main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["main-branch", "skip-main-branch"])
        .help("No main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("skip-main-branch")
        .long("skip-main-branch")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["main-branch", "no-main-branch"])
        .help("Skip the configured main branch of the pacticipant as the version for which you want to check the verification results"))
    .arg(Arg::new("to-environment")
        .long("to-environment")
        .value_name("ENVIRONMENT")
        .help("The environment into which the pacticipant(s) are to be deployed"))
    .arg(Arg::new("to")
        .long("to")
        .value_name("TO")
        .help("The tag that represents the branch or environment of the integrated applications for which you want to check the verification result status."))
        .args(crate::cli::add_output_arguments(["json", "table"].to_vec(), "table"))
    .arg(Arg::new("retry-while-unknown")
        .long("retry-while-unknown")
        .value_name("TIMES")
        .help("The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running)"))
    .arg(Arg::new("retry-interval")
        .long("retry-interval")
        .value_name("SECONDS")
        .help("The time between retries in seconds. Use in conjuction with --retry-while-unknown"))
    .arg(Arg::new("dry-run")
        .long("dry-run")
        .num_args(0)
        .conflicts_with_all(&["skip-dry-run", "no-dry-run"])
        .action(clap::ArgAction::SetTrue)
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))
    .arg(Arg::new("no-dry-run")
        .long("no-dry-run")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["skip-dry-run", "dry-run"])
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))
    .arg(Arg::new("skip-dry-run")
        .long("skip-dry-run")
        .num_args(0)
        .action(clap::ArgAction::SetTrue)
        .conflicts_with_all(&["no-dry-run", "dry-run"])
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_DEPLOY_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))

.args(add_broker_auth_arguments())
.args(crate::cli::add_verbose_arguments())
}
fn add_can_i_merge_subcommand() -> Command {
    Command::new("can-i-merge")
    .about("Checks if the specified pacticipant version is compatible with the configured main branch of each of the pacticipants with which it is integrated.")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .num_args(0..=1)
        .help("The pacticipant name. Use once for each pacticipant being checked."))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version. Must be entered after the --pacticipant that it relates to."))
        .args(crate::cli::add_output_arguments(["json", "table"].to_vec(), "table"))
    .arg(Arg::new("retry-while-unknown")
        .long("retry-while-unknown")
        .value_name("TIMES")
        .default_value("0")
        .help("The number of times to retry while there is an unknown verification result (ie. the provider verification is likely still running)"))
    .arg(Arg::new("retry-interval")
        .long("retry-interval")
        .value_name("SECONDS")
        .default_value("10")
        .help("The time between retries in seconds. Use in conjuction with --retry-while-unknown"))
    .arg(Arg::new("dry-run")
        .long("dry-run")
        .help("When dry-run is enabled, always exit process with a success code. Can also be enabled by setting the environment variable PACT_BROKER_CAN_I_MERGE_DRY_RUN=true. This mode is useful when setting up your CI/CD pipeline for the first time, or in a 'break glass' situation where you need to knowingly deploy what Pact considers a breaking change. For the second scenario, it is recommended to use the environment variable and just set it for the build required to deploy that particular version, so you don't accidentally leave the dry run mode enabled."))

.args(crate::cli::add_verbose_arguments())
}
fn add_create_or_update_pacticipant_subcommand() -> Command {
    Command::new("create-or-update-pacticipant")
        .about("Create or update pacticipant by name")
        .args(add_broker_auth_arguments())
        .arg(
            Arg::new("name")
                .long("name")
                .value_name("NAME")
                .required(true)
                .help("Pacticipant name"),
        )
        .arg(
            Arg::new("display-name")
                .long("display-name")
                .value_name("DISPLAY_NAME")
                .help("Display name"),
        )
        .arg(
            Arg::new("main-branch")
                .long("main-branch")
                .value_name("MAIN_BRANCH")
                .help("The main development branch of the pacticipant repository"),
        )
        .arg(
            Arg::new("repository-url")
                .long("repository-url")
                .value_name("REPOSITORY_URL")
                .help("The repository URL of the pacticipant"),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text"].to_vec(),
            "text",
        ))
        .args(crate::cli::add_verbose_arguments())
}
fn add_describe_pacticipant_subcommand() -> Command {
    Command::new("describe-pacticipant")
        .about("Describe a pacticipant")
        .args(add_broker_auth_arguments())
        .arg(
            Arg::new("name")
                .long("name")
                .value_name("NAME")
                .required(true)
                .help("Pacticipant name"),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text"].to_vec(),
            "text",
        ))
        .args(crate::cli::add_verbose_arguments())
}
fn add_list_pacticipants_subcommand() -> Command {
    Command::new("list-pacticipants")
        .about("List pacticipants")
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_output_arguments(
            ["json", "text"].to_vec(),
            "text",
        ))
        .args(crate::cli::add_verbose_arguments())
}
fn add_create_webhook_subcommand() -> Command {
    Command::new("create-webhook")
    .about("Create a webhook")
    .arg(Arg::new("url")
        .value_name("URL")
        .required(true)
        .help("Webhook URL"))
    .arg(Arg::new("request")
        .short('X')
        .long("request")
        .value_name("METHOD")
        .help("Webhook HTTP method"))
    .arg(Arg::new("header")
        .short('H')
        .long("header")
        .value_name("one two three")
        .num_args(0..=1)
        .help("Webhook Header"))
    .arg(Arg::new("data")
        .short('d')
        .long("data")
        .value_name("DATA")
        .help("Webhook payload"))
    .arg(Arg::new("user")
        // .short('u')
        .long("user")
        .value_name("USER")
        .help("Webhook basic auth username and password eg. username:password"))
    .arg(Arg::new("consumer")
        .long("consumer")
        .value_name("CONSUMER")
        .help("Consumer name"))
    .arg(Arg::new("consumer-label")
        .long("consumer-label")
        .value_name("CONSUMER_LABEL")
        .help("Consumer label, mutually exclusive with consumer name"))
    .arg(Arg::new("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("Provider name"))
    .arg(Arg::new("provider-label")
        .long("provider-label")
        .value_name("PROVIDER_LABEL")
        .help("Provider label, mutually exclusive with provider name"))
    .arg(Arg::new("description")
        .long("description")
        .value_name("DESCRIPTION")
        .help("Webhook description"))
    .arg(Arg::new("contract-content-changed")
        .long("contract-content-changed")
        .help("Trigger this webhook when the pact content changes"))
    .arg(Arg::new("contract-published")
        .long("contract-published")
        .help("Trigger this webhook when a pact is published"))
    .arg(Arg::new("provider-verification-published")
        .long("provider-verification-published")
        .help("Trigger this webhook when a provider verification result is published"))
    .arg(Arg::new("provider-verification-failed")
        .long("provider-verification-failed")
        .help("Trigger this webhook when a failed provider verification result is published"))
    .arg(Arg::new("provider-verification-succeeded")
        .long("provider-verification-succeeded")
        .help("Trigger this webhook when a successful provider verification result is published"))
    .arg(Arg::new("contract-requiring-verification-published")
        .long("contract-requiring-verification-published")
        .help("Trigger this webhook when a contract is published that requires verification"))
    .arg(Arg::new("team-uuid")
        .long("team-uuid")
        .value_name("UUID")
        .help("UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)"))

.args(add_broker_auth_arguments())
.args(crate::cli::add_verbose_arguments())
}
fn add_create_or_update_webhook_subcommand() -> Command {
    Command::new("create-or-update-webhook")
    .about("Create or update a webhook")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("url")
        .value_name("URL")
        .required(true)
        .help("Webhook URL"))
    .arg(Arg::new("uuid")
        .long("uuid")
        .value_name("UUID")
        .required(true)
        .help("Specify the uuid for the webhook"))
    .arg(Arg::new("request")
        .short('X')
        .long("request")
        .value_name("METHOD")
        .help("Webhook HTTP method"))
    .arg(Arg::new("header")
        .short('H')
        .long("header")
        .value_name("one two three")
        .num_args(0..=1)
        .help("Webhook Header"))
    .arg(Arg::new("data")
        .short('d')
        .long("data")
        .value_name("DATA")
        .help("Webhook payload"))
    .arg(Arg::new("user")
        // .short('u')
        .long("user")
        .value_name("USER")
        .help("Webhook basic auth username and password eg. username:password"))
    .arg(Arg::new("consumer")
        .long("consumer")
        .value_name("CONSUMER")
        .help("Consumer name"))
    .arg(Arg::new("consumer-label")
        .long("consumer-label")
        .value_name("CONSUMER_LABEL")
        .help("Consumer label, mutually exclusive with consumer name"))
    .arg(Arg::new("provider")
        .long("provider")
        .value_name("PROVIDER")
        .help("Provider name"))
    .arg(Arg::new("provider-label")
        .long("provider-label")
        .value_name("PROVIDER_LABEL")
        .help("Provider label, mutually exclusive with provider name"))
    .arg(Arg::new("description")
        .long("description")
        .value_name("DESCRIPTION")
        .help("Webhook description"))
    .arg(Arg::new("contract-content-changed")
        .long("contract-content-changed")
        .help("Trigger this webhook when the pact content changes"))
    .arg(Arg::new("contract-published")
        .long("contract-published")
        .help("Trigger this webhook when a pact is published"))
    .arg(Arg::new("provider-verification-published")
        .long("provider-verification-published")
        .help("Trigger this webhook when a provider verification result is published"))
    .arg(Arg::new("provider-verification-failed")
        .long("provider-verification-failed")
        .help("Trigger this webhook when a failed provider verification result is published"))
    .arg(Arg::new("provider-verification-succeeded")
        .long("provider-verification-succeeded")
        .help("Trigger this webhook when a successful provider verification result is published"))
    .arg(Arg::new("contract-requiring-verification-published")
        .long("contract-requiring-verification-published")
        .help("Trigger this webhook when a contract is published that requires verification"))
    .arg(Arg::new("team-uuid")
        .long("team-uuid")
        .value_name("UUID")
        .help("UUID of the PactFlow team to which the webhook should be assigned (PactFlow only)"))
        .args(crate::cli::add_verbose_arguments())
}
fn add_test_webhook_subcommand() -> Command {
    Command::new("test-webhook")
        .about("Test a webhook")
        .arg(
            Arg::new("uuid")
                .long("uuid")
                .value_name("UUID")
                .num_args(1)
                .required(true)
                .help("Specify the uuid for the webhook"),
        )
        .args(add_broker_auth_arguments())
        .args(crate::cli::add_verbose_arguments())
}
fn add_delete_branch_subcommand() -> Command {
    Command::new("delete-branch")
    .about("Deletes a pacticipant branch. Does not delete the versions or pacts/verifications associated with the branch, but does make the pacts inaccessible for verification via consumer versions selectors or WIP pacts.")
    .args(add_broker_auth_arguments())
    .arg(Arg::new("branch")
        .long("branch")
        .value_name("BRANCH")
        .required(true)
        .help("The pacticipant branch name"))
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .help("The name of the pacticipant that the branch belongs to"))
.args(crate::cli::add_verbose_arguments())
}
fn add_create_version_tag_subcommand() -> Command {
    Command::new("create-version-tag")
        .about("Add a tag to a pacticipant version")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The pacticipant name"),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version"),
        )
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .value_name("TAG")
                .num_args(0..=1)
                .help("Tag name for pacticipant version. Can be specified multiple times"),
        )
        .arg(
            Arg::new("auto-create-version")
                .long("auto-create-version")
                .help("Automatically create the pacticipant version if it does not exist"),
        )
        .arg(
            Arg::new("tag-with-git-branch")
                .short('g')
                .long("tag-with-git-branch")
                .help("Tag pacticipant version with the name of the current git branch"),
        )
}
fn add_describe_version_subcommand() -> Command {
    Command::new("describe-version")
    .about("Describes a pacticipant version. If no version or tag is specified, the latest version is described.")
    .arg(Arg::new("pacticipant")
        .short('a')
        .long("pacticipant")
        .value_name("PACTICIPANT")
        .required(true)
        .help("The name of the pacticipant that the version belongs to"))
    .arg(Arg::new("version")
        .short('e')
        .long("version")
        .value_name("VERSION")
        .help("The pacticipant version number"))
    .arg(Arg::new("latest")
        .short('l')
        .long("latest")
        .value_name("TAG")
        .help("Describe the latest pacticipant version. Optionally specify a TAG to describe the latest version with the specified tag"))
        .args(crate::cli::add_output_arguments(["json", "table", "id"].to_vec(), "table"))
}
fn add_create_or_update_version_subcommand() -> Command {
    Command::new("create-or-update-version")
        .about("Create or update pacticipant version by version number")
        .arg(
            Arg::new("pacticipant")
                .short('a')
                .long("pacticipant")
                .value_name("PACTICIPANT")
                .required(true)
                .help("The pacticipant name"),
        )
        .arg(
            Arg::new("version")
                .short('e')
                .long("version")
                .value_name("VERSION")
                .required(true)
                .help("The pacticipant version number"),
        )
        .arg(
            Arg::new("branch")
                .long("branch")
                .value_name("BRANCH")
                .help("The repository branch name"),
        )
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .value_name("TAG")
                .num_args(0..=1)
                .help("Tag name for pacticipant version. Can be specified multiple times"),
        )
        .args(crate::cli::add_output_arguments(
            ["json", "text"].to_vec(),
            "text",
        ))
}
fn add_generate_uuid_subcommand() -> Command {
    Command::new("generate-uuid")
        .about("Generate a UUID for use when calling create-or-update-webhook")
}

pub fn run(args: &ArgMatches) {
    match args.subcommand() {
        Some(("publish", args)) => {
            let res = pact_publish::handle_matches(args);
            match res {
                Ok(_) => {
                    let res = pact_publish::publish_pacts(args);
                    match res {
                        Ok(_res) => {
                            std::process::exit(0);
                        }
                        Err(err) => {
                            std::process::exit(err);
                        }
                    }
                }
                Err(err) => {
                    std::process::exit(err);
                }
            }
        }
        Some(("list-latest-pact-versions", args)) => {
            // setup client with broker url and credentials
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            let broker_details = BrokerDetails {
                url: broker_url.clone(),
                auth: Some(auth),
            };
            let default_output: String = "text".to_string();
            let output_arg: &String = args.get_one::<String>("output").unwrap_or(&default_output);
            let output = match output_arg.as_str() {
                "json" => OutputType::Json,
                "table" => OutputType::Table,
                "pretty" => OutputType::Pretty,
                _ => OutputType::Text,
            };

            let verbose = args.get_flag("verbose");
            let res = list_latest_pact_versions(&broker_details, output, verbose);
            if let Err(err) = res {
                handle_error(err);
            }
        }
        Some(("create-environment", args)) => {
            let name = args.get_one::<String>("name");
            let display_name = args.get_one::<String>("display-name");
            let production = args.get_flag("production");
            let contact_name = args.get_one::<String>("contact-name");
            let contact_email_address = args.get_one::<String>("contact-email-address");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));

                let mut payload = json!({});
                payload["production"] = serde_json::Value::Bool(production);
                if let Some(name) = name {
                    payload["name"] = serde_json::Value::String(name.to_string());
                } else {
                    println!("❌ {}", Colour::Red.paint("Name is required"));
                    std::process::exit(1);
                }
                if let Some(contact_name) = contact_name {
                    payload["contacts"] = serde_json::Value::Array(vec![{
                        let mut map = serde_json::Map::new();
                        map.insert(
                            "name".to_string(),
                            serde_json::Value::String(contact_name.to_string()),
                        );
                        serde_json::Value::Object(map)
                    }]);
                }
                if let Some(display_name) = display_name {
                    payload["displayName"] = serde_json::Value::String(display_name.to_string());
                }
                if let Some(contact_email_address) = contact_email_address {
                    if payload["contacts"].is_array() {
                        let contacts = payload["contacts"].as_array_mut().unwrap();
                        let contact = contacts.get_mut(0).unwrap();
                        let contact_map = contact.as_object_mut().unwrap();
                        contact_map.insert(
                            "email".to_string(),
                            serde_json::Value::String(contact_email_address.to_string()),
                        );
                    } else {
                        payload["contacts"] = serde_json::Value::Array(vec![{
                            let mut map = serde_json::Map::new();
                            map.insert(
                                "email".to_string(),
                                serde_json::Value::String(contact_email_address.to_string()),
                            );
                            serde_json::Value::Object(map)
                        }]);
                    }
                }
                let res = hal_client
                    .post_json(&(broker_url + "/environments"), &payload.to_string())
                    .await;

                let default_output: String = "text".to_string();
                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                match res {
                    Ok(res) => {
                        if output == "pretty" {
                            let json = serde_json::to_string_pretty(&res).unwrap();
                            println!("{}", json);
                        } else if output == "json" {
                            println!("{}", serde_json::to_string(&res).unwrap());
                        } else if output == "id" {
                            println!("{}", res["uuid"].to_string().trim_matches('"'));
                        } else {
                            let uuid = res["uuid"].to_string();
                            println!(
                                "✅ Created {} environment in the Pact Broker with UUID {}",
                                Colour::Green.paint(name.unwrap()),
                                Colour::Green.paint(uuid.trim_matches('"'))
                            );
                        }
                        std::process::exit(0);
                    }
                    Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                    }
                }
            })
        }
        Some(("update-environment", args)) => {
            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
            let name = args.get_one::<String>("name");
            let display_name = args.get_one::<String>("display-name");
            let production = args.get_flag("production");
            let contact_name = args.get_one::<String>("contact-name");
            let contact_email_address = args.get_one::<String>("contact-email-address");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));

                let mut payload = json!({});
                payload["uuid"] = serde_json::Value::String(uuid);
                payload["production"] = serde_json::Value::Bool(production);
                if let Some(name) = name {
                    payload["name"] = serde_json::Value::String(name.to_string());
                } else {
                    println!("❌ {}", Colour::Red.paint("Name is required"));
                    std::process::exit(1);
                }
                if let Some(contact_name) = contact_name {
                    payload["contacts"] = serde_json::Value::Array(vec![{
                        let mut map = serde_json::Map::new();
                        map.insert(
                            "name".to_string(),
                            serde_json::Value::String(contact_name.to_string()),
                        );
                        serde_json::Value::Object(map)
                    }]);
                }
                if let Some(display_name) = display_name {
                    payload["displayName"] = serde_json::Value::String(display_name.to_string());
                }
                if let Some(contact_email_address) = contact_email_address {
                    if payload["contacts"].is_array() {
                        let contacts = payload["contacts"].as_array_mut().unwrap();
                        let contact = contacts.get_mut(0).unwrap();
                        let contact_map = contact.as_object_mut().unwrap();
                        contact_map.insert(
                            "email".to_string(),
                            serde_json::Value::String(contact_email_address.to_string()),
                        );
                    } else {
                        payload["contacts"] = serde_json::Value::Array(vec![{
                            let mut map = serde_json::Map::new();
                            map.insert(
                                "email".to_string(),
                                serde_json::Value::String(contact_email_address.to_string()),
                            );
                            serde_json::Value::Object(map)
                        }]);
                    }
                }
                let res = hal_client
                    .post_json(&(broker_url + "/environments"), &payload.to_string())
                    .await;

                let default_output = "text".to_string();
                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                let columns = vec![
                    "ID",
                    "NAME",
                    "DISPLAY NAME",
                    "PRODUCTION",
                    "CONTACT NAME",
                    "CONTACT EMAIL ADDRESS",
                ];
                let names = vec![
                    vec!["id"],
                    vec!["name"],
                    vec!["displayName"],
                    vec!["production"],
                    vec!["contactName"],
                    vec!["contactEmailAddress"],
                ];
                match res {
                    Ok(res) => {
                        if output == "pretty" {
                            let json = serde_json::to_string_pretty(&res).unwrap();
                            println!("{}", json);
                        } else if output == "json" {
                            println!("{}", serde_json::to_string(&res).unwrap());
                        } else if output == "id" {
                            println!("{}", res["uuid"].to_string().trim_matches('"'));
                        } else if output == "table" {
                            let table = crate::cli::pact_broker::main::utils::generate_table(
                                &res, columns, names,
                            );
                            println!("{table}");
                        } else {
                            let uuid = res["uuid"].to_string();
                            println!(
                                "✅ Updated {} environment in the Pact Broker with UUID {}",
                                Colour::Green.paint(name.unwrap()),
                                Colour::Green.paint(uuid.trim_matches('"'))
                            );
                        }

                        std::process::exit(0);
                    }
                    Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                    }
                }
            })
        }
        Some(("describe-environment", args)) => {
            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));
                let res = hal_client
                    .fetch(&(broker_url + "/environments/" + &uuid))
                    .await;

                let default_output = "text".to_string();
                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                match res {
                    Ok(res) => {
                        if output == "pretty" {
                            let json = serde_json::to_string_pretty(&res).unwrap();
                            println!("{}", json);
                        } else if output == "json" {
                            println!("{}", serde_json::to_string(&res).unwrap());
                        } else {
                            let res_uuid = res["uuid"].to_string();
                            let res_name = res["name"].to_string();
                            let res_display_name = res["displayName"].to_string();
                            let res_production = res["production"].to_string();
                            let res_created_at = res["createdAt"].to_string();
                            let res_contacts = res["contacts"].as_array();

                            println!("✅");
                            println!("UUID {}", Colour::Green.paint(res_uuid.trim_matches('"')));
                            println!("Name: {}", Colour::Green.paint(res_name.trim_matches('"')));
                            println!(
                                "Display Name: {}",
                                Colour::Green.paint(res_display_name.trim_matches('"'))
                            );
                            println!(
                                "Production: {}",
                                Colour::Green.paint(res_production.trim_matches('"'))
                            );
                            println!(
                                "Created At: {}",
                                Colour::Green.paint(res_created_at.trim_matches('"'))
                            );
                            if let Some(contacts) = res_contacts {
                                println!("Contacts:");
                                for contact in contacts {
                                    println!(" - Contact:");
                                    if let Some(name) = contact["name"].as_str() {
                                        println!("  - Name: {}", name);
                                    }
                                    if let Some(email) = contact["email"].as_str() {
                                        println!("  - Email: {}", email);
                                    }
                                }
                            }
                        }

                        std::process::exit(0);
                    }
                    Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                    }
                }
            })
        }
        Some(("delete-environment", args)) => {
            let uuid = args.get_one::<String>("uuid").unwrap().to_string();
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));
                let res = hal_client
                    .clone()
                    .fetch(&(broker_url.clone() + "/environments/" + &uuid))
                    .await;
                match res {
                    Ok(_) => {
                        let name = res.clone().unwrap()["name"].to_string();
                        let res = hal_client
                            .clone()
                            .delete(&(broker_url.clone() + "/environments/" + &uuid))
                            .await;
                        match res {
                            Ok(_) => {
                                println!(
                                    "✅ Deleted environment {} from the Pact Broker with UUID {}",
                                    Colour::Green.paint(name),
                                    Colour::Green.paint(uuid.trim_matches('"'))
                                );
                                std::process::exit(0);
                            }
                            Err(err) => match err {
                                PactBrokerError::LinkError(error) => {
                                    println!("❌ {}", Colour::Red.paint(error));
                                    std::process::exit(1);
                                }
                                PactBrokerError::ContentError(error) => {
                                    println!("❌ {}", Colour::Red.paint(error));
                                    std::process::exit(1);
                                }
                                PactBrokerError::IoError(error) => {
                                    println!("❌ {}", Colour::Red.paint(error));
                                    std::process::exit(1);
                                }
                                PactBrokerError::NotFound(error) => {
                                    println!("❌ {}", Colour::Red.paint(error));
                                    std::process::exit(1);
                                }
                                PactBrokerError::ValidationError(errors) => {
                                    for error in errors {
                                        println!("❌ {}", Colour::Red.paint(error));
                                    }
                                    std::process::exit(1);
                                }
                                _ => {
                                    println!("❌ {}", Colour::Red.paint(err.to_string()));
                                    std::process::exit(1);
                                }
                            },
                        }
                    }
                    Err(err) => match err {
                        PactBrokerError::LinkError(error) => {
                            println!("❌ {}", Colour::Red.paint(error));
                            std::process::exit(1);
                        }
                        PactBrokerError::ContentError(error) => {
                            println!("❌ {}", Colour::Red.paint(error));
                            std::process::exit(1);
                        }
                        PactBrokerError::IoError(error) => {
                            println!("❌ {}", Colour::Red.paint(error));
                            std::process::exit(1);
                        }
                        PactBrokerError::NotFound(error) => {
                            println!("❌ {}", Colour::Red.paint(error));
                            std::process::exit(1);
                        }
                        PactBrokerError::ValidationError(errors) => {
                            for error in errors {
                                println!("❌ {}", Colour::Red.paint(error));
                            }
                            std::process::exit(1);
                        }
                        _ => {
                            println!("❌ {}", Colour::Red.paint(err.to_string()));
                            std::process::exit(1);
                        }
                    },
                }
            })
        }
        Some(("list-environments", args)) => {
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));
                let res = hal_client.fetch(&(broker_url + "/environments/")).await;

                let default_output = "text".to_string();
                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                match res {
                    Ok(res) => {
                        if output == "pretty" {
                            let json = serde_json::to_string_pretty(&res).unwrap();
                            println!("{}", json);
                        } else if output == "json" {
                            println!("{}", serde_json::to_string(&res).unwrap());
                        } else {
                            let mut table = Table::new();

                            #[derive(Debug, serde::Deserialize)]
                            struct Environment {
                                uuid: String,
                                name: String,
                                #[serde(rename = "displayName")]
                                display_name: String,
                                production: bool,
                                #[serde(rename = "createdAt")]
                                created_at: String,
                            }

                            table.load_preset(UTF8_FULL).set_header(vec![
                                "UUID",
                                "NAME",
                                "DISPLAY NAME",
                                "PRODUCTION",
                                "CREATED AT",
                            ]);

                            if let Some(embedded) = res["_embedded"].as_object() {
                                if let Some(environments) = embedded["environments"].as_array() {
                                    for environment in environments {
                                        let environment: Environment =
                                            serde_json::from_value(environment.clone()).unwrap();
                                        table.add_row(vec![
                                            environment.uuid,
                                            environment.name,
                                            environment.display_name,
                                            environment.production.to_string(),
                                            environment.created_at,
                                        ]);
                                    }
                                }
                            }

                            println!("{table}");
                        }

                        std::process::exit(0);
                    }
                    Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                    }
                }
            })
        }
        Some(("record-deployment", args)) => {
            let version = args.get_one::<String>("version");
            let pacticipant = args.get_one::<String>("pacticipant");
            let environment = args.get_one::<String>("environment");
            let application_instance = args.get_one::<String>("application-instance");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient =
                    HALClient::with_url(&broker_url, Some(auth.clone()));

            let res = hal_client.clone()
                .fetch(
                    &(broker_url.clone()
                        + "/pacticipants/"
                        + &pacticipant.unwrap()
                        + "/versions/"
                        + &version.unwrap()),
                )
                .await;

            #[derive(Debug, Deserialize, Serialize)]
            struct PacticipantVersions {
                _embedded: Embedded,
                _links: Links,
                #[serde(rename = "createdAt")]
                created_at: String,
                number: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Links {
                #[serde(rename = "self")]
                self_link: Link,
                #[serde(rename = "pb:pacticipant")]
                pacticipant_link: Link,
                #[serde(rename = "pb:tag")]
                tag_link: Link,
                #[serde(rename = "pb:latest-verification-results-where-pacticipant-is-consumer")]
                latest_verification_results_link: Link,
                #[serde(rename = "pb:pact-versions")]
                pact_versions: Vec<Link>,
                #[serde(rename = "pb:record-deployment")]
                record_deployment: Vec<Link>,
                #[serde(rename = "pb:record-release")]
                record_release: Vec<Link>,
                curies: Vec<Curies>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Link {
                href: String,
                name: Option<String>,
                title: Option<String>,
                templated: Option<bool>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Curies {
                name: String,
                href: String,
                templated: bool
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Embedded {
                #[serde(rename = "branchVersions")]
                branch_versions: Vec<BranchVersion>,
                tags: Vec<Tag>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct BranchVersion {
                _links: VersionLinks,
                latest: bool,
                name: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Tag {
                _links: TagLinks,
                name: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct VersionLinks {
                #[serde(rename = "self")]
                self_link: Link,
                name: Option<String>,
                title: Option<String>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct TagLinks {
                #[serde(rename = "self")]
                self_link: Link,
                name: Option<String>,
                title: Option<String>,
            }
            match res {
                Ok(res) => {

                    let result: Result<PacticipantVersions, serde_json::Error> = serde_json::from_value(res);
                    match result {
                        Ok(data) => {
                        match data._links.record_deployment.iter().find(|x| x.name == Some(environment.unwrap().to_string())) {
                            Some(link) => {
                                let link_record_deployment_href = &link.href;

                                // println!("✅ Found environment {} with {}", Colour::Green.paint(environment.unwrap()), Colour::Green.paint(link_record_deployment_href.clone()));

                                // <- "POST /pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/json\r\nHost: localhost:9292\r\nContent-Length: 44\r\n\r\n"
                                // <- "{\"applicationInstance\":\"foo\",\"target\":\"foo\"}"


                                let mut payload = json!({});
                                payload["target"] = serde_json::Value::String(environment.unwrap().to_string());
                                if let Some(application_instance) = application_instance {
                                    payload["applicationInstance"] = serde_json::Value::String(application_instance.to_string());
                                }
                                let res: Result<Value, PactBrokerError> = hal_client.clone().post_json(&(link_record_deployment_href.clone()), &payload.to_string()).await;
                                let default_output = "text".to_string();
                                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                                match res {
                                    Ok(res) => {
                                            if output == "pretty" {
                                                let json = serde_json::to_string_pretty(&res).unwrap();
                                                println!("{}", json);
                                            } else if output == "json" {
                                                println!("{}", serde_json::to_string(&res).unwrap());
                                            } else if output == "id" {
                                                println!("{}", res["uuid"].to_string().trim_matches('"'));
                                            }
                                            else {
                                                // let uuid = res["uuid"].to_string();
                                                println!("✅ Recorded deployment of {} version {} to {} environment{} in the Pact Broker.", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(version.unwrap()),Colour::Green.paint(environment.unwrap()), application_instance.map(|instance| format!(" (application instance {})", Colour::Green.paint(instance))).unwrap_or_default());
                                                // println!("✅ Created {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

                                            }
                                        std::process::exit(0);
                                    }
                                    Err(err) => {
                                        match err {
                                            // TODO process output based on user selection
                                            PactBrokerError::LinkError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ContentError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::IoError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::NotFound(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ValidationError(errors) => {
                                                for error in errors {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                }
                                                std::process::exit(1);
                                            }
                                            _ => {
                                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }
                                        }
                            None => {
                                println!("❌ Environment {} does not exist", Colour::Red.paint(environment.unwrap()));
                                std::process::exit(1);
                            }}
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            println!("❌ {}", Colour::Red.paint("Failed to record deployment"));
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                }
        }})
        }
        Some(("record-undeployment", args)) => {
            // 1. Check broker index link for connection
            // <- "GET /? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 2. Call environments and check the specified enviroment exists, get the environment link
            // <- "GET /environments? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 3. Call the environment link and check the specified version exists, get the version link
            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 4. Call the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App link, and check our app is currently deployed
            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 5. perform a patch request to the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 link to set currentlyDeployed to false
            // <- "PATCH /deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/merge-patch+json\r\nHost: localhost:9292\r\nContent-Length: 27\r\n\r\n"
            // <- "{\"currentlyDeployed\":false}"

            let pacticipant = args.get_one::<String>("pacticipant");
            let environment = args.get_one::<String>("environment");
            let _application_instance = args.get_one::<String>("application-instance");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
            let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));


            let res = hal_client.clone()
                .fetch(&(broker_url.clone() + "/"))
                .await;
            match res {
                Ok(_) => {
                    // Handle success
                }
                Err(err) => {
                    handle_error(err);
                }
            }

            #[derive(Debug, serde::Deserialize)]
            struct Environment {
                uuid: String,
                name: String,
                #[serde(rename = "displayName")]
                display_name: String,
                production: bool,
                #[serde(rename = "createdAt")]
                created_at: String,
            }

            let res = hal_client.clone()
                .fetch(&(broker_url.clone() + "/environments?"))
                .await;
                match res {
                    Ok(response) => {
                        let environments: Vec<Environment> = response["_embedded"]["environments"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|env| serde_json::from_value(env.clone()).unwrap())
                            .collect();
                        let environment_exists = environments.iter().any(|env| env.name == environment.clone().unwrap().to_string());
                        if environment_exists {
                            let environment_uuid = &environments.iter().find(|env| env.name == environment.clone().unwrap().to_string()).unwrap().uuid;
                            // Use environment_uuid in step 3

                            // 3. Call the environment link and check the specified version exists, get the version link
                            let res = hal_client.clone()
                            .fetch(&(broker_url.clone() + "/environments/" + &environment_uuid + "?"))
                            .await;
                        match res {
                            Ok(result) => {
                                // print!("✅ Environment found");
                                // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                // print!("Result JSON: {:#?}", result);
                                // todo - handle application instance

                                let currently_deployed_link = result["_links"]["pb:currently-deployed-deployed-versions"]["href"].as_str().unwrap();
                                let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));

                                let res = hal_client.clone()
                                    .fetch(&(currently_deployed_link.to_owned() + &pacticipant_query))
                                    .await;
                                match res {
                                    Ok(result) => {
                                        // Handle success
                                        // print!("🧹 Found currently deployed versions");
                                        // print!("Result JSON: {:#?}", result);
                                        if let Some(embedded) = result["_embedded"].as_object() {
                                            if let Some(deployed_versions) = embedded["deployedVersions"].as_array() {
                                                if deployed_versions.len() == 0 {
                                                    print!("❌ No currently deployed versions in {} environment", environment.unwrap());
                                                    std::process::exit(1);
                                                }
                                                for deployed_version in deployed_versions {
                                                    let pacticipant_name = deployed_version["_embedded"]["pacticipant"]["name"].as_str().unwrap();
                                                    if pacticipant_name == pacticipant.unwrap() {
                                                        let self_href = deployed_version["_links"]["self"]["href"].as_str().unwrap();
                                                        // Send a patch request with the user's payload to selfHref
                                                        // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                        // print!("🧹 Sending a patch request to {}", self_href);
                                                        let mut payload = json!({});
                                                        payload["currentlyDeployed"] = serde_json::Value::Bool(false);
                                                        // let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));
                                                        let res = hal_client.clone().patch_json(self_href, &payload.to_string()).await;
                                                        match res {
                                                            Ok(_) => {
                                                                // Handle success
                                                                print!("✅ ♻️ Undeployed {} from {} environment", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(environment.unwrap()));
                                                            }
                                                            Err(err) => {
                                                                handle_error(err);
                                                            }
                                                        }
                                                    } else {
                                                        print!("❌ No currently deployed versions found for {} in {} environment" ,pacticipant.unwrap(), environment.unwrap());
                                                        std::process::exit(1);
                                                    }
                                                }
                                            } else {
                                                print!("❌ No currently deployed versions in {} environment", environment.unwrap());
                                                std::process::exit(1);
                                            }
                                            }
                                            else {
                                                print!("❌ Could not process hal relation link");
                                                std::process::exit(1);
                                            }
                                    }
                                    Err(err) => {
                                        handle_error(err);
                                    }
                                }
                            }
                            Err(err) => {
                                handle_error(err);
                            }
                        }
                        } else {
                            println!("❌ Environment not found");
                            std::process::exit(1);
                        }
                    }
                    Err(err) => {
                        handle_error(err);
                        }
                    }
                })
        }
        Some(("record-release", args)) => {
            // 1. Check broker index link for connection
            // 2, Check version exists "GET /pacticipants/{pacticipant}/versions/{versions}?
            // "{\"number\":\"5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\",\"createdAt\":\"2024-03-17T07:11:23+00:00\",\"_embedded\":{\"branchVersions\":[{\"name\":\"main\",\"latest\":true,\"_links\":{\"self\":{\"title\":\"Branch version\",\"name\":\"main\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/branches/main/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"}}}],\"tags\":[{\"name\":\"main\",\"_links\":{\"self\":{\"title\":\"Tag\",\"name\":\"main\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/tags/main\"}}}]},\"_links\":{\"self\":{\"title\":\"Version\",\"name\":\"5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"},\"pb:pacticipant\":{\"title\":\"Pacticipant\",\"name\":\"Example App\",\"href\":\"http://localhost:9292/pacticipants/Example%20App\"},\"pb:tag\":{\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/tags/{tag}\",\"title\":\"Get, create or delete a tag for this pacticipant version\",\"templated\":true},\"pb:latest-verification-results-where-pacticipant-is-consumer\":{\"title\":\"Latest verification results for consumer version\",\"href\":\"http://localhost:9292/verification-results/consumer/Example%20App/version/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/latest\"},\"pb:pact-versions\":[{\"title\":\"Pact\",\"name\":\"Pact between Example App (5556b8149bf8bac76bc30f50a8a2dd4c22c85f30) and Example API\",\"href\":\"http://localhost:9292/pacts/provider/Example%20API/consumer/Example%20App/version/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30\"}],\"pb:record-deployment\":[{\"title\":\"Record deployment to Production\",\"name\":\"production\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b\"},{\"title\":\"Record deployment to Test\",\"name\":\"test\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/cf7dcfdb-3645-4b16-b2f7-7ecb4b6045e0\"}],\"pb:record-release\":[{\"title\":\"Record release to Production\",\"name\":\"production\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/released-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b\"},{\"title\":\"Record release to Test\",\"name\":\"test\",\"href\":\"http://localhost:9292/pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/released-versions/environment/cf7dcfdb-3645-4b16-b2f7-7ecb4b6045e0\"}],\"curies\":[{\"name\":\"pb\",\"href\":\"http://localhost:9292/doc/{rel}?context=version\",\"templated\":true}]}}"
            // 3. Find the pb:record-release link for the specified environment
            // 4. Send a POST request to the pb:record-release link with an empty payload
            // 5. Handle the response

            let version = args.get_one::<String>("version");
            let pacticipant = args.get_one::<String>("pacticipant");
            let environment = args.get_one::<String>("environment");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
            let hal_client: HALClient =
                HALClient::with_url(&broker_url, Some(auth.clone()));

            let res = hal_client.clone()
                .fetch(
                    &(broker_url.clone()
                        + "/pacticipants/"
                        + &pacticipant.unwrap()
                        + "/versions/"
                        + &version.unwrap()),
                )
                .await;

            #[derive(Debug, Deserialize, Serialize)]
            struct PacticipantVersions {
                _embedded: Embedded,
                _links: Links,
                #[serde(rename = "createdAt")]
                created_at: String,
                number: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Links {
                #[serde(rename = "self")]
                self_link: Link,
                #[serde(rename = "pb:pacticipant")]
                pacticipant_link: Link,
                #[serde(rename = "pb:tag")]
                tag_link: Link,
                #[serde(rename = "pb:latest-verification-results-where-pacticipant-is-consumer")]
                latest_verification_results_link: Link,
                #[serde(rename = "pb:pact-versions")]
                pact_versions: Vec<Link>,
                #[serde(rename = "pb:record-deployment")]
                record_deployment: Vec<Link>,
                #[serde(rename = "pb:record-release")]
                record_release: Vec<Link>,
                curies: Vec<Curies>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Link {
                href: String,
                name: Option<String>,
                title: Option<String>,
                templated: Option<bool>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Curies {
                name: String,
                href: String,
                templated: bool
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Embedded {
                #[serde(rename = "branchVersions")]
                branch_versions: Vec<BranchVersion>,
                tags: Vec<Tag>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct BranchVersion {
                _links: VersionLinks,
                latest: bool,
                name: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct Tag {
                _links: TagLinks,
                name: String,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct VersionLinks {
                #[serde(rename = "self")]
                self_link: Link,
                name: Option<String>,
                title: Option<String>,
            }

            #[derive(Debug, Deserialize, Serialize)]
            struct TagLinks {
                #[serde(rename = "self")]
                self_link: Link,
                name: Option<String>,
                title: Option<String>,
            }
            match res {
                Ok(res) => {

                    let result: Result<PacticipantVersions, serde_json::Error> = serde_json::from_value(res);
                    match result {
                        Ok(data) => {
                        match data._links.record_release.iter().find(|x| x.name == Some(environment.unwrap().to_string())) {
                            Some(link) => {
                                let record_release_href = &link.href;

                                // println!("✅ Found environment {} with {}", Colour::Green.paint(environment.unwrap()), Colour::Green.paint(link_record_deployment_href.clone()));

                                // <- "POST /pacticipants/Example%20App/versions/5556b8149bf8bac76bc30f50a8a2dd4c22c85f30/deployed-versions/environment/c540ce64-5493-48c5-ab7c-28dae27b166b HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/json\r\nHost: localhost:9292\r\nContent-Length: 44\r\n\r\n"
                                // <- "{\"applicationInstance\":\"foo\",\"target\":\"foo\"}"


                                let payload = json!({});
                                let res: Result<Value, PactBrokerError> = hal_client.clone().post_json(&(record_release_href.clone()), &payload.to_string()).await;
                                let default_output = "text".to_string();
                                let output = args.get_one::<String>("output").unwrap_or(&default_output);
                                match res {
                                    Ok(res) => {
                                            if output == "pretty" {
                                                let json = serde_json::to_string_pretty(&res).unwrap();
                                                println!("{}", json);
                                            } else if output == "json" {
                                                println!("{}", serde_json::to_string(&res).unwrap());
                                            } else if output == "id" {
                                                println!("{}", res["uuid"].to_string().trim_matches('"'));
                                            }
                                            else {
                                                // let uuid = res["uuid"].to_string();
                                                println!("✅ Recorded release of {} version {} to {} environment in the Pact Broker.", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(version.unwrap()),Colour::Green.paint(environment.unwrap()));
                                                // println!("✅ Created {} environment in the Pact Broker with UUID {}", Colour::Green.paint(name.unwrap()), Colour::Green.paint(uuid.trim_matches('"')));

                                            }
                                        std::process::exit(0);
                                    }
                                    Err(err) => {
                                        match err {
                                            // TODO process output based on user selection
                                            PactBrokerError::LinkError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ContentError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::IoError(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::NotFound(error) => {
                                                println!("❌ {}", Colour::Red.paint(error));
                                                std::process::exit(1);
                                            }
                                            PactBrokerError::ValidationError(errors) => {
                                                for error in errors {
                                                    println!("❌ {}", Colour::Red.paint(error));
                                                }
                                                std::process::exit(1);
                                            }
                                            _ => {
                                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                }
                                        }
                            None => {
                                println!("❌ Environment {} does not exist", Colour::Red.paint(environment.unwrap()));
                                std::process::exit(1);
                            }}
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            println!("❌ {}", Colour::Red.paint("Failed to record release"));
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                        match err {
                            // TODO process output based on user selection
                            PactBrokerError::LinkError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ContentError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::IoError(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::NotFound(error) => {
                                println!("❌ {}", Colour::Red.paint(error));
                                std::process::exit(1);
                            }
                            PactBrokerError::ValidationError(errors) => {
                                for error in errors {
                                    println!("❌ {}", Colour::Red.paint(error));
                                }
                                std::process::exit(1);
                            }
                            _ => {
                                println!("❌ {}", Colour::Red.paint(err.to_string()));
                                std::process::exit(1);
                            }
                        }
                }
        }})
        }
        Some(("record-support-ended", args)) => {
            // 1. Check broker index link for connection
            // <- "GET /? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 2. Call environments and check the specified enviroment exists, get the environment link
            // <- "GET /environments? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 3. Call the environment link and check the specified version exists, get the version link
            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b? HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 4. Call the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App link, and check our app is currently deployed
            // <- "GET /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/currently-deployed?pacticipant=Example+App HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nHost: localhost:9292\r\n\r\n"
            // -> "HTTP/1.1 200 OK\r\n"
            // 5. perform a patch request to the /environments/c540ce64-5493-48c5-ab7c-28dae27b166b/deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 link to set currentlyDeployed to false
            // <- "PATCH /deployed-versions/9b756f93-19a2-4ca7-ae36-c0b917ac1f21 HTTP/1.1\r\nAccept: application/hal+json\r\nUser-Agent: Ruby\r\nContent-Type: application/merge-patch+json\r\nHost: localhost:9292\r\nContent-Length: 27\r\n\r\n"
            // <- "{\"currentlyDeployed\":false}"

            let pacticipant = args.get_one::<String>("pacticipant");
            let environment = args.get_one::<String>("environment");
            let broker_url = get_broker_url(args);
            let auth = get_auth(args);
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));


                let res = hal_client.clone()
                    .fetch(&(broker_url.clone() + "/"))
                    .await;
                match res {
                    Ok(_) => {
                        // Handle success
                    }
                    Err(err) => {
                        handle_error(err);
                    }
                }

                #[derive(Debug, serde::Deserialize)]
                struct Environment {
                    uuid: String,
                    name: String,
                    #[serde(rename = "displayName")]
                    display_name: String,
                    production: bool,
                    #[serde(rename = "createdAt")]
                    created_at: String,
                    }

                let res = hal_client.clone()
                    .fetch(&(broker_url.clone() + "/environments?"))
                    .await;
                    match res {
                        Ok(response) => {
                            let environments: Vec<Environment> = response["_embedded"]["environments"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|env| serde_json::from_value(env.clone()).unwrap())
                                .collect();
                            let environment_exists = environments.iter().any(|env| env.name == environment.clone().unwrap().to_string());
                            if environment_exists {
                                let environment_uuid = &environments.iter().find(|env| env.name == environment.clone().unwrap().to_string()).unwrap().uuid;
                                // Use environment_uuid in step 3

                                // 3. Call the environment link and check the specified version exists, get the version link
                                let res = hal_client.clone()
                                .fetch(&(broker_url.clone() + "/environments/" + &environment_uuid + "?"))
                                .await;
                            match res {
                                Ok(result) => {
                                    // print!("✅ Environment found");
                                    // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                    // print!("Result JSON: {:#?}", result);
                                    // todo - handle application instance

                                    let currently_supported_released_link = result["_links"]["pb:currently-supported-released-versions"]["href"].as_str().unwrap();
                                    let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));

                                    let res = hal_client.clone()
                                        .fetch(&(currently_supported_released_link.to_owned() + &pacticipant_query))
                                        .await;
                                    match res {
                                        Ok(result) => {
                                            // Handle success
                                            // print!("🧹 Found currently deployed versions");
                                            // print!("Result JSON: {:#?}", result);
                                            if let Some(embedded) = result["_embedded"].as_object() {
                                                if let Some(released_versions) = embedded["releasedVersions"].as_array() {
                                                    if released_versions.len() == 0 {
                                                        print!("❌ No currently released versions in {} environment", environment.unwrap());
                                                        std::process::exit(1);
                                                    }
                                                    for released_version in released_versions {
                                                        let pacticipant_name = released_version["_embedded"]["pacticipant"]["name"].as_str().unwrap();
                                                        if pacticipant_name == pacticipant.unwrap() {
                                                            let self_href = released_version["_links"]["self"]["href"].as_str().unwrap();
                                                            // Send a patch request with the user's payload to selfHref
                                                            // print!("🧹 Undeploying {} from {} environment", pacticipant.unwrap(), environment.unwrap());
                                                            // print!("🧹 Sending a patch request to {}", self_href);
                                                            let mut payload = json!({});
                                                            payload["currentlySupported"] = serde_json::Value::Bool(false);
                                                            // let pacticipant_query = format!("?pacticipant={}", urlencoding::encode(pacticipant.unwrap()));
                                                            let res = hal_client.clone().patch_json(self_href, &payload.to_string()).await;
                                                            match res {
                                                                Ok(_) => {
                                                                    // Handle success
                                                                    print!("✅ ♻️ Recorded support ended {} from {} environment", Colour::Green.paint(pacticipant.unwrap()), Colour::Green.paint(environment.unwrap()));
                                                                }
                                                                Err(err) => {
                                                                    handle_error(err);
                                                                }
                                                            }
                                                        } else {
                                                            print!("❌ No currently released versions found for {} in {} environment" ,pacticipant.unwrap(), environment.unwrap());
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                } else {
                                                    print!("❌ No currently released versions in {} environment", environment.unwrap());
                                                    std::process::exit(1);
                                                }
                                                }
                                            else {
                                                print!("❌ Could not process hal relation link");
                                                std::process::exit(1);
                                            }
                                        }
                                        Err(err) => {
                                            handle_error(err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    handle_error(err);
                                }
                            }
                            } else {
                                println!("❌ Environment not found");
                                std::process::exit(1);
                            }
                        }
                        Err(err) => {
                            handle_error(err);
                            }
                        }
            })
        }
        Some(("can-i-deploy", args)) => {
            let pacticipant = args.get_one::<String>("pacticipant");
            let version = args.get_one::<String>("version");
            let _ignore = args.get_flag("ignore");
            let latest = args.get_flag("latest");
            let branch = args.get_one::<String>("branch");
            let _main_branch = args.get_flag("main-branch");
            let _no_main_branch = args.get_flag("no-main-branch");
            let _skip_main_branch = args.get_flag("skip-main-branch");
            let to_environment = args.get_one::<String>("to-environment");
            let to = args.get_one::<String>("to");
            let _retry_while_unknown = args.get_one::<String>("retry-while-unknown");
            let _retry_interval = args.get_one::<String>("retry-interval");
            let dry_run = args.get_flag("dry-run");
            let _no_dry_run = args.get_flag("no-dry-run");
            let _skip_dry_run = args.get_flag("skip-dry-run");

            let broker_url = get_broker_url(args);
            let auth = get_auth(args);

            #[derive(Debug, serde::Deserialize)]
            struct Summary {
                deployable: Option<bool>,
                reason: String,
                success: u32,
                failed: u32,
                unknown: u32,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Notice {
                #[serde(rename = "type")]
                notice_type: String,
                text: String,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Version {
                number: String,
                branch: String,
                branches: Vec<Branch>,
                #[serde(rename = "branchVersions")]
                branch_versions: Vec<BranchVersion>,
                environments: Vec<Environment>,
                _links: Links,
                tags: Vec<Tag>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Branch {
                name: String,
                latest: Option<bool>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct BranchVersion {
                name: String,
                latest: Option<bool>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Environment {
                uuid: String,
                name: String,
                #[serde(rename = "displayName")]
                display_name: String,
                production: Option<bool>,
                #[serde(rename = "createdAt")]
                created_at: String,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Links {
                #[serde(rename = "self")]
                self_link: SelfLink,
            }

            #[derive(Debug, serde::Deserialize)]
            struct SelfLink {
                href: String,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Tag {
                name: String,
                latest: Option<bool>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Consumer {
                name: String,
                version: Option<Version>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Provider {
                name: String,
                version: Option<Version>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Pact {
                #[serde(rename = "createdAt")]
                created_at: String,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct VerificationResult {
                success: Option<bool>,
                #[serde(rename = "verifiedAt")]
                verified_at: Option<String>,
                _links: Links,
            }

            #[derive(Debug, serde::Deserialize)]
            struct MatrixItem {
                consumer: Consumer,
                provider: Provider,
                pact: Pact,
                #[serde(rename = "verificationResult")]
                verification_result: Option<VerificationResult>,
            }

            #[derive(Debug, serde::Deserialize)]
            struct Data {
                summary: Summary,
                notices: Vec<Notice>,
                matrix: Vec<MatrixItem>,
            }

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let hal_client: HALClient = HALClient::with_url(&broker_url, Some(auth.clone()));
                // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][latest]=true&q[][branch]=foo&latestby=cvp&latest=true".to_string();
                // let matrix_href_path = "/matrix?q[][pacticipant]=Example+App&q[][version]=5556b8149bf8bac76bc30f50a8a2dd4c22c85f30&latestby=cvp&latest=true".to_string();
                let mut matrix_href_path = "/matrix?".to_string();

                if let Some(pacticipant) = pacticipant {
                    matrix_href_path.push_str(&format!("q[][pacticipant]={}&", pacticipant));
                }

                if let Some(version) = version {
                    matrix_href_path.push_str(&format!("q[][version]={}&", version));
                }

                if latest {
                    matrix_href_path.push_str("latest=true&");
                }

                if let Some(branch) = branch {
                    matrix_href_path.push_str(&format!("q[][branch]={}&", branch));
                }

                if let Some(to_environment) = to_environment {
                    matrix_href_path.push_str(&format!("environment={}&", to_environment));
                }
                if let Some(to) = to {
                    matrix_href_path.push_str(&format!("tag={}&", to));
                }

                matrix_href_path.push_str("latestby=cvp");
                // query the hal relation link to get the latest pact versions
                let res = hal_client
                    .clone()
                    .fetch(&(broker_url.clone() + &matrix_href_path))
                    .await;
                match res {
                    Ok(res) => {
                        // handle user args for additional processing
                        let output: Result<Option<&String>, clap::parser::MatchesError> =
                            args.try_get_one::<String>("output");

                        // render result
                        match output {
                            Ok(Some(output)) => {
                                if output == "json" {
                                    let json: String = serde_json::to_string(&res.clone()).unwrap();
                                    println!("{}", json);
                                } else if output == "table" {
                                    // println!("{:?}", res.clone());

                                    let data: Data =
                                        match serde_json::from_str(&res.clone().to_string()) {
                                            Ok(data) => data,
                                            Err(err) => {
                                                println!(
                                                    "❌ {}",
                                                    Colour::Red.paint(err.to_string())
                                                );
                                                Data {
                                                    summary: Summary {
                                                        deployable: Some(false),
                                                        success: 0,
                                                        failed: 0,
                                                        reason: "No summary found".to_string(),
                                                        unknown: 1,
                                                    },
                                                    notices: Vec::new(),
                                                    matrix: Vec::new(),
                                                }
                                            }
                                        };

                                    if data.matrix.len() > 0 {
                                        let mut table = Table::new();

                                        table.load_preset(UTF8_FULL).set_header(vec![
                                            "CONSUMER",
                                            "C.VERSION",
                                            "PROVIDER",
                                            "P.VERSION",
                                            "SUCCESS?",
                                            "RESULT",
                                        ]);
                                        for matrix_item in data.matrix {
                                            let verification_result = &matrix_item
                                                .verification_result
                                                .map(|result| {
                                                    result.success.unwrap_or(false).to_string()
                                                })
                                                .unwrap_or_else(|| "false".to_string());

                                            table.add_row(vec![
                                                matrix_item.consumer.name,
                                                matrix_item
                                                    .consumer
                                                    .version
                                                    .map(|result| result.number.to_string())
                                                    .unwrap_or_else(|| "unknown".to_string()),
                                                matrix_item.provider.name,
                                                matrix_item
                                                    .provider
                                                    .version
                                                    .map(|result| result.number.to_string())
                                                    .unwrap_or_else(|| "unknown".to_string()),
                                                verification_result.to_string(),
                                                verification_result.to_string(),
                                            ]);
                                        }
                                        println!("{table}");
                                    }

                                    if data.notices.len() > 0 {
                                        for notice in data.notices {
                                            if notice.notice_type == "warning" {
                                                println!("⚠️ {}", Colour::Yellow.paint(notice.text));
                                            } else if notice.notice_type == "error" {
                                                println!("❌ {}", Colour::Red.paint(notice.text));
                                            } else {
                                                println!("📌 {}", Colour::Green.paint(notice.text));
                                            }
                                        }
                                    }
                                    if data.summary.deployable.unwrap_or(false) {
                                        let computer_says_yes = Colour::Green.paint("\\o/");
                                        println!(r"✅ Computer says yes {}", computer_says_yes);
                                    } else {
                                        let computer_says_no = Colour::Red.paint("¯\\_(ツ)_/¯");
                                        println!(r"❌ Computer says no {}", computer_says_no);
                                        if dry_run == true {
                                            println!("📌 Dry run enabled, suppressing failing exit code");
                                            std::process::exit(0);
                                        }
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(res) => {
                                println!("no output match provided {:?}", res);
                                std::process::exit(1);
                            }
                            _ => {
                                println!("{:?}", res.clone());
                            }
                        }
                    }
                    Err(res) => {
                        handle_error(res);
                    }
                }
            })
        }
        Some(("can-i-merge", _args)) => {
            // Handle can-i-merge command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("create-or-update-pacticipant", _args)) => {
            // Handle create-or-update-pacticipant command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("describe-pacticipant", _args)) => {
            // Handle describe-pacticipants command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("list-pacticipants", _args)) => {
            // Handle list-pacticipants command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("create-webhook", _args)) => {
            // Handle create-webhook command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("create-or-update-webhook", _args)) => {
            // Handle create-or-update-webhook command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("test-webhook", _args)) => {
            // Handle test-webhook command

            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("delete-branch", _args)) => {
            // Handle delete-branch command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("create-version-tag", _args)) => {
            // Handle create-version-tag command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("describe-version", _args)) => {
            // Handle describe-version command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("create-or-update-version", _args)) => {
            // Handle create-or-update-version command
            println!("Unimplemented");
            std::process::exit(1);
        }
        Some(("generate-uuid", _args)) => {
            println!("{}", uuid::Uuid::new_v4());
        }
        _ => {
            println!("⚠️  No option provided, try running pact-broker --help");
        }
    }
}
