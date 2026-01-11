#!/usr/bin/env bash

# find mc proc
PID=$(pgrep -f minecraft | head -n 1)

# get absolute path of lib
LIB="./target/debug/librust_mc_client.so"
ABS=$(realpath "$LIB")

# use gdb to load it
gdb -q -n -batch \
  -ex "attach $PID" \
  -ex "call (void*)dlopen(\"$ABS\", 1)" \
  -ex "detach" \
  -ex "quit"

