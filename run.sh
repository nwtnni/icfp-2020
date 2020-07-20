#!/bin/sh

RUST_LOG="icfp=debug,submit=info" /solution/target/release/submit "$@" || echo "run error code: $?"
