#include <stdio.h>

extern void ffi();

int main() {
    printf("App calling from C ...\n");
    ffi();
    printf("Call succesfull - exiting main.\n");

    return 0;
}
