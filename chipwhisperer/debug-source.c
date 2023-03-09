#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#define DEBUGGING

// Put your handler definitions here.
// uint8_t set_key(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t* buf);

uint8_t TRIGGER_SETUP = 0;
uint8_t UART_SETUP = 0;
uint8_t PLATFORM_SETUP = 0;
uint8_t SS_SETUP = 0;

void platform_init() {
    printf("Initiated platform!\n");
    PLATFORM_SETUP = 1;
}
void init_uart() {
    if (PLATFORM_SETUP == 0) {
        printf("Tried to setup UART without Platform setup");
        exit(-1);
    }
    printf("Initiated UArt!\n");
    UART_SETUP = 1;
}
void trigger_setup() {
    if (UART_SETUP == 0) {
        printf("Tried to setup Trigger without UART setup");
        exit(-1);
    }
    printf("Trigger Setup!\n");
    TRIGGER_SETUP = 1;
}
void trigger_high() {
    if (TRIGGER_SETUP == 0) {
        printf("Tried to set trigger to high without trigger setup");
        exit(-1);
    }
    printf("Trigger put at High!\n");
}
void trigger_low() {
    if (TRIGGER_SETUP == 0) {
        printf("Tried to set trigger to low without trigger setup");
        exit(-1);
    }
    printf("Trigger put at Low!\n");
}

void simpleserial_init() {
    if (TRIGGER_SETUP == 0) {
        printf("Tried to setup SimpleSerial without Trigger setup");
        exit(-1);
    }
    printf("Initiated SimpleSerial!\n");
    SS_SETUP = 1;
}
void simpleserial_put(uint8_t cmd, uint8_t len, uint8_t* buf) {
    printf("Put '%c' on SS with len %u", cmd, len);
    printf("Data: [\n");

    for (int i = 0; i < len; ++i) {
        printf("%x, ", buf[i]);
        if (i != 0 && i != (len - 1) && i % 3 == 0)
            printf("\n");
    }

    printf("\n]\n");
}
void simpleserial_addcmd(uint8_t cmd, uint8_t len, uint8_t (*f)(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t *buf)) {
    printf("Listening for '%c' on SS with preferred len %u\n", cmd, len);
}
void simpleserial_get(uint8_t cmd, uint8_t len, uint8_t (*f)(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t *buf)) {
    // Put your debug code here
    

    exit(0);
}