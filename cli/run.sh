#!/bin/zsh

# set -e

cargo build

../target/debug/cli $1
