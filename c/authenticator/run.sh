#!/bin/bash
gcc -O2 -s main.c -L../../target/release -lsafe_authenticator -o authenticator.out
LD_LIBRARY_PATH=../../target/release:${LD_LIBRARY_PATH} ./authenticator.out
