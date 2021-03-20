#!/usr/bin/env bash

if ! command -v svg-term &> /dev/null; then
    echo "Command 'svg-term' not found. Please install with 'npm install -g svg-term-cli'."
    exit
fi

set -e

cargo build
svg-term "--command=./demo-script.sh" "--out=demo.svg" --from=10 --window --width=80 --height=10 --no-cursor
