#!/usr/bin/env bash

## This script will set up all the build artifacts and assist in saving time
## when working through the workshop. Otherwise the compile time will be a cost
## for each new section.
##
## Usage: start_section.sh <section_directory>
## section_directory - the directory name for the section

script_location=$(dirname $0)
section_number=$1

if (( $# != 1 )); then
    echo "Please provide the section directory name"
    echo "Usage: start_section.sh <section_directory>"
    exit 1
fi

echo "Starting section $section_number"

# Download and compile. If this is already done it will be quick
cd ${script_location}/ci
cargo check
cd ..

echo "Build artifacts initialized. Copying to section $1"

# Copy the built artifacts to the current working directory
cp -r ci/target $1
cd $1

echo "Let's begin hacking!"