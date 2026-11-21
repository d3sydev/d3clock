#!/usr/bin/env bash
set -e

if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo is not installed. install rustup first: https://rustup.rs"
    exit 1
fi

echo "building d3clock..."
cargo build --release

mkdir -p "$HOME/.local/bin"
cp target/release/d3clock "$HOME/.local/bin/d3clock"
chmod +x "$HOME/.local/bin/d3clock"

echo "installed to ~/.local/bin/d3clock"
case ":$PATH:" in
    *":$HOME/.local/bin:"*) ;;
    *) echo "note: add ~/.local/bin to your PATH to run 'd3clock' from anywhere" ;;
esac
