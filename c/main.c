#include <stdio.h>

extern void ffi();

int main() {
    printf("Calling ffi...\n");
    ffi();
    printf("Finished Calling ffi...\n");
    return 0;
}
