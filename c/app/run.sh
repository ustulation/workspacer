#!/bin/bash
gcc -O2 -s main.c -L../../target/release -lsafe_app -o app.out
LD_LIBRARY_PATH=../../target/release:${LD_LIBRARY_PATH} ./app.out
