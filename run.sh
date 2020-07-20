#!/bin/sh

RUST_LOG="icfp=debug" /solution/target/release/submit "$@" || echo "run error code: $?"
