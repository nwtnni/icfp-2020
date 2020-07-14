#!/bin/sh

/solution/target/release/icfp "$@" || echo "run error code: $?"
