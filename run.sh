#!/bin/sh

RUST_BACKTRACE="full" RUST_LOG="icfp=debug,submit=info" /solution/target/release/submit "$@" || echo "run error code: $?"
