#!/bin/bash

err_report() {
    echo "errexit on line $(caller)" >&2
}

trap err_report ERR

echo hello | grep foo

# shfmt -w -i 4 test_trap.sh 
