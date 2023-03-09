#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include "rustheader.h"

int main(void) {
    volatile unsigned int res = add(32, 32);

    printf("%u", res);

    while(1) {

    }
}