# WKSL - a wake and sleep utility

An experiment in writing a small CLI utility in Rust. The program lets you wake a machine on 
your local network up from sleep, and put it back down to sleep again, all from the command line.

## Installation

This assumes you have a recent version of rust installed, if not, go to [rustup.rs](https://rustup.rs) 
and follow instructions.

 Clone this repo somewhere

     $ git clone https://github.com/hravnx/wksl.git

Navigate into the resulting `wksl` folder and run

     $ cargo install --path .

This should build and install the `wksl` executable into the Cargo bin folder,
which is normally in `$HOME/.cargo/bin` - you should make sure this is in your path.

Confirm the installation worked by running 
    
    $ wksl help

This should print some help information.

## Configuration

The program depends on a small [TOML](https://toml.io/) file for configuration. 
It looks for `$HOME/.config/wksl/config.toml` by default, but this can be overridden
on the command line by giving the `-c` or `--config-path` argument, like so:

    $ wksl --config-path ./my/path/to/wksl.toml <... rest of your args here ...> 

Look at the [example](example-config.toml) file in the root of the repo for details. 

## License

MIT License - See [LICENSE](LICENSE) file for details.

Copyright (c) 2021 Henrik Ravn
