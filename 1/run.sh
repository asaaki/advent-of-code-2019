#!/bin/sh
cargo fmt && cargo test && cargo run -- input/modules.txt
