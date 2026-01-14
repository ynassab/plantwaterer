#!/bin/sh
set -e

cargo build --release
sudo cp target/release/plantwaterer /usr/local/bin/plantwaterer
sudo chmod +x /usr/local/bin/plantwaterer

