#include <stdio.h>
#include "libhello_c_world.h"

int main() {
    const uint8_t *string = get_string(9);
    printf("%s", string);

    const char *string2 = to_uppercase("Hello World!"); 
    printf("%s", string2);

}