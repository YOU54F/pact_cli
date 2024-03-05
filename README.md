# Clap cli

A very simple clap cli app

## 1

Run the top level app

```console
$ clap_cli
? 2
clap_cli 0.1.0
Author Name
A Very simple App with multiple subcommands

USAGE:
    clap_cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    app1    Run app 1
    app2    Run app 1
    help    Print this message or the help of the given subcommand(s)

```

## 2

Run the sub command app1, which requires a name argument, not-required by clap

```console
$ clap_cli app1 
? 1
error in processing : No name provided

```

## 3

Run the sub command app2, which requires a name argument, required by clap

```console
$ clap_cli app2
? 2
error: The following required arguments were not provided:
    <NAME>

USAGE:
    clap_cli app2 <NAME>

For more information try --help

```


## 4

Run the sub command app1, with a provided name argument

It is non-required by clap, so the consumer must expect an option and match it

```console
$ clap_cli app1 foo
you provided foo to app1

```

## 5

Run the sub command app2, with a provided name argument

It is required, so the consumer can expect a string

```console
$ clap_cli app2 bar
you provided bar to app2

```
