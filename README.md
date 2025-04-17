# secret

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/clementi/secret/ci.yml?branch=main)](https://github.com/clementi/secret/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/secret-gen)](https://crates.io/crates/secret-gen)

![Secret](media/logo.png)

Command-line password/passphrase generator.

## Installation

You can install secret either by using Cargo, or with Homebrew, or by downloading a binary from the Releases page.

### Cargo

Run this in your shell:

```sh
cargo install secret-gen
```

### Homebrew

```sh
brew tap clementi/homebrew-secret
brew install secret
```

### Binaries

See the [Releases](https://github.com/clementi/secret/releases) page.

## Usage

```
Usage: secret [OPTIONS] <COMMAND>

Commands:
  token   Generate a token (string) of random characters
  phrase  Generate a passphrase of random words
  help    Print this message or the help of the given subcommand(s)

Options:
  -n, --number <COUNT>  Number of tokens or phrases to generate [default: 1]
  -h, --help            Print help information
  -V, --version         Print version information
```

### Create a Password

The `token` command has this usage:

```
Usage: secret token [OPTIONS]

Options:
  -l, --length <LENGTH>  Length of token [default: 20]
      --alpha-lower      Use lowercase letters
      --alpha-upper      Use uppercase letters
      --alpha            Use lowercase and uppercase letters (equivalent to --alpha-lower --alpha-upper)
      --numeric          Use numeric characters
      --alphanumeric     Use alphanumeric characters (equivalent to --alpha --numeric)
      --symbols          Use symbols
  -a, --all              Use everything (equivalent to --alphanumeric --symbols) [default if no other character set is provided]
  -h, --help             Print help information
  -V, --version          Print version information
```

### Create a Passphrase

The `phrase` command has this usage:

```
Usage: secret phrase [OPTIONS]

Options:
  -l, --length <LENGTH>        Length of phrase (in words) [default: 4]
  -s, --separator <SEPARATOR>  Word separator [default: " "]
  -h, --help                   Print help information
  -V, --version                Print version information
```
