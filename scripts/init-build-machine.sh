#!/bin/bash

# Check sudo
if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

# Install required rust components
apt-get update
apt-get install -y curl protobuf-compiler sed libssl-dev libudev-dev

sudo -u $SUDO_USER curl https://sh.rustup.rs -sSf | sh -s -- -y
source /home/$SUDO_USER/.cargo/env
cargo install cargo-deb 
cargo install --force cbindgen