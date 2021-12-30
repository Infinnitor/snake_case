#!/bin/bash
cargo build
cd target/debug

# This file is just to run the program with commandline arguments
# You cannot do this with 'cargo run'
run="./snake_case "
for var in "$@"
do
	run+=" "
	run+=$var
done

echo $run
eval $run
