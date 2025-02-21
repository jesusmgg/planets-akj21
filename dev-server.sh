#!/bin/sh

live-server \
--port=42069 \
--cors \
--verbose \
--no-browser \
--ignore=deps,incremental,examples,build \
target/wasm32-unknown-unknown/debug
