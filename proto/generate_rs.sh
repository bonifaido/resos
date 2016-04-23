#!/bin/bash

brew install protobuf
cargo install protobuf

PATH="$HOME/.cargo/bin:$PATH"
VERSION=0.28.1

cd proto/mesos/v1
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/mesos.proto
cd executor
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/executor/executor.proto
cd ../scheduler
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/scheduler/scheduler.proto
cd ../../../..

protoc --rust_out src proto/mesos/v1/mesos.proto
protoc --rust_out src --proto_path=proto proto/mesos/v1/executor/executor.proto
protoc --rust_out src --proto_path=proto proto/mesos/v1/scheduler/scheduler.proto
