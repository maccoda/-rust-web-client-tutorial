#!/usr/bin/env bash
set -e

script=$(dirname $0)

for dir in $script/*
do
    [ -d "${dir}" ] || continue # if not a directory, skip
    echo "Testing $dir"
    cd $dir
    if [ -f "Cargo.toml" ]; then
        cargo test
        cd solution
        echo "Checking solution for $dir"
        cargo test
        cd ..
    fi
    cd ..

done