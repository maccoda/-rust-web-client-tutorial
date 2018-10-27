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
        for sol in $(ls -d solution*)
        do
            cd $sol
            echo "Checking solution for $dir"
            cargo test
            cd ..
        done
    fi
    cd ..

done