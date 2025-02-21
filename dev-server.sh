#!/bin/sh

live-server \
--port=42069 \
--cors \
--verbose \
--no-browser \
--ignore=deps,incremental,examples,build \
--watch=akj-21.wasm \
target/wasm32-unknown-unknown/debug
