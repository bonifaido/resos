# rust-mesos

[![Join the chat at https://gitter.im/bonifaido/rust-mesos](https://badges.gitter.im/bonifaido/rust-mesos.svg)](https://gitter.im/bonifaido/rust-mesos?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Build Status](https://travis-ci.org/bonifaido/rust-mesos.png?branch=master)](https://travis-ci.org/bonifaido/rust-mesos)

Rust library for [Apache Mesos](http://mesos.apache.org).
It is based on the upcoming Scheduler and Executor (not yet implemented in this library) HTTP API in Mesos, see:
  - https://github.com/apache/mesos/blob/master/docs/executor-http-api.md
  - https://github.com/apache/mesos/blob/master/docs/scheduler_http_api.md

####*Warning:* This library is in a very early stage, it is not recomended for production and all APIs are subject to change.

## Prerequisites

- [Apache Mesos](http://mesos.apache.org)
- [Rust](http://rust-lang.org)
 
Rust Mesos depends on the MesosMasterInfo in Zookeeper to be published in JSON format and thus Mesos master or the upcoming 0.24 (see [this commit](https://github.com/apache/mesos/commit/18e1351b3c5c24f7f65be66ee56889a6378ba97f)).

## Building

This project is always built using [cargo](http://doc.crates.io) against the latest stable, beta, and nightly.

```bash
$ cargo build
```

The protobuf code was generated using
[rust-protobuf](https://github.com/stepancheg/rust-protobuf).

## Running the example framework

Your mesos master and slave must be running and register themselves to `zk://127.0.0.1:2181/mesos`.

Then you can execute the [example framework](examples/framework_example.rs) with

```bash
$ cargo run --example framework_example
```

or if you prefer verbose logging inside the mesos crate with

```bash
$ RUST_LOG="mesos=debug" cargo run --example framework_example
```
