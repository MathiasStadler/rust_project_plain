#!/bin/bash
trap exit 1 ERR
MyVar="${RUST_PROJECT:-default_value}"
echo "${MyVar}";
