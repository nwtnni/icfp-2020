#!/bin/sh

/solution/target/release/submit "$@" || echo "run error code: $?"
