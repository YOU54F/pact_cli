# Clap cli

A very simple clap cli app

## pact_cli

```console
$ pact_cli
? 2
pact_cli 0.1.0
Author Name
Pact CLI

USAGE:
    pact_cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help           Print this message or the help of the given subcommand(s)
    pact-broker    
    pactflow       

```

## pact_broker client

```console
$ pact_cli pact-broker
? 2
pact_cli-pact-broker 0.1.0
Pact Foundation

USAGE:
    pact_cli pact-broker <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    publish    

```
## pactflow client

```console
$ pact_cli pactflow
? 2
pact_cli-pactflow 0.1.0
PactFlow

USAGE:
    pact_cli pactflow <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help                         Print this message or the help of the given subcommand(s)
    publish-provider-contract    

```

## publish-provider-contract

```console
$ pact_cli pactflow publish-provider-contract
? 2
error: The following required arguments were not provided:
    --contentFile <CONTENT_FILE>
    --broker-base-url <BROKER_BASE_URL>
    --broker-token <BROKER_TOKEN>
    --provider <PROVIDER>
    --provider_app_version <PROVIDER_APP_VERSION>

USAGE:
    pact_cli pactflow publish-provider-contract [OPTIONS] --contentFile <CONTENT_FILE> --broker-base-url <BROKER_BASE_URL> --broker-token <BROKER_TOKEN> --provider <PROVIDER> --provider_app_version <PROVIDER_APP_VERSION>

For more information try --help

```
