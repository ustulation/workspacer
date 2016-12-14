#include <stdio.h>

extern void ffi();

int main() {
    printf("Authenticator calling from C ...\n");
    ffi();
    printf("Call succesfull - exiting main.\n");

    return 0;
}
