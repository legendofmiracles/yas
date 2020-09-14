#!/usr/bin/env bash
set -e
cargo build
sudo chown root target/debug/yas
sudo chmod u+s target/debug/yas
