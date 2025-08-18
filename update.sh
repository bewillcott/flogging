#!/usr/bin/bash

# Clean directories
echo Clean directories
rm -r ./docs
rm -r ./guide/src/api
rm -r ./guide/src/coverage
# rm -r ./guide/src/extras
rm -r ./guide/src/**.html

# Rebuild 'api' documentation
echo Rebuild 'api' documentation
cargo doc --no-deps --workspace
mv ./target/doc ./guide/src/api

# Rebuild llvm-cov-pretty files
echo Rebuild llvm-cov-pretty files
cargo llvm-cov nextest --json | llvm-cov-pretty --output-dir guide/src/coverage
# mv ./target/llvm-cov-pretty ./guide/src/coverage

# Copy in 'extras'
# echo Copy in 'extras'
# mkdir ./guide/src/extras
# ln -s ../../../CHANGELOG.md ./guide/src/extras
# ln -s ../../../RELEASELOG.md ./guide/src/extras

# Build the book
echo Build the book
mdbook build guide --dest-dir ../docs
