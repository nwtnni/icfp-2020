#!/bin/sh

RUST_LOG="info" /solution/target/release/submit "$@" || echo "run error code: $?"
