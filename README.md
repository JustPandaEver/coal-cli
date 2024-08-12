DONATE ME 
```sh
FqX4B7prPuZa1HjmJbWQtLH6GYYqUTT9wzWr55XQTbyE
```

OR

```sh
0x6510cB12a2B6bb49cDbfbd23E6D40ba521f48f35
```


# COAL CLI

A command line interface for COAL cryptocurrency mining.

## Install

To install the CLI, use [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/JustPandaEver/coal-cli && cd coal-cli && cargo install --path .
```


### Dependencies
If you run into issues during installation, please install the following dependencies for your operating system and try again:

#### Linux
```
sudo apt-get install openssl pkg-config libssl-dev
```

#### MacOS (using [Homebrew](https://brew.sh/))
```
brew install openssl pkg-config

# If you encounter issues with OpenSSL, you might need to set the following environment variables:
export PATH="/usr/local/opt/openssl/bin:$PATH"
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
```

#### Windows (using [Chocolatey](https://chocolatey.org/))
```
choco install openssl pkgconfiglite
```

## Build

To build the codebase from scratch, checkout the repo and use cargo to build:

```sh
cargo build --release
```


## Usage


```sh
coal --rpc url --cores cpucore --jito
```


## Help

You can use the `-h` flag on any command to pull up a help menu with documentation:

```sh
coal -h
```