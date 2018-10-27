#!/usr/bin/env bash
set -e

script=$(dirname $0)
test_cmd="cargo test -q"

for dir in $script/*
do
    [ -d "${dir}" ] || continue # if not a directory, skip
    echo "Testing $dir"
    cd $dir
    if [ -f "Cargo.toml" ]; then
        $test_cmd
        for sol in $(ls -d solution*)
        do
            cd $sol
            echo "Checking $sol for $dir"
            $test_cmd
            cd ..
        done
    fi
    cd ..

done