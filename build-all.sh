#!/bin/sh

args="$@"

cd types && cargo build $args && cd .. || exit 1
cd parser && cargo build $args && cd .. || exit 1
cd engine && cargo build $args && cd .. || exit 1
cd jsbinding && cargo build $args && cd .. || exit 1
