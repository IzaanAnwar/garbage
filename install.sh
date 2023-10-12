#!/bin/bash

#Build The Project
cargo build --release

#Install the built binary to usr/local/bin
echo "Installing in usr/local/bin"
sudo cp ./target/release/garbage /usr/local/bin
echo "Installed Successfully"

