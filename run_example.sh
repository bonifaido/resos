#!/bin/bash

cargo test

RUST_LOG=info RUST_BACKTRACE=1 target/debug/examples/framework_example