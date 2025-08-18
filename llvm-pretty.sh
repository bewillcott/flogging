#!/usr/bin/bash

#
# Used by "bacon llvm-cov"
#
# Update lvm-cov-pretty output
cargo llvm-cov test --json | llvm-cov-pretty --output-dir guide/src/coverage
