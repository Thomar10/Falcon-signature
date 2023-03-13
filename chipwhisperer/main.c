/*
 * SimpleSerial V2 Template C code
 * Can be freely used to implement ChipWhisperer target binaries.
 *
 * Date: 14th March 2021
 * https://github.com/coastalwhite/simpleserial-c-template/blob/main/main.c
 */

#include <stdint.h>
#include <stdlib.h>
#include "rustlib.h"

//#define SS_VER 2.1

#include "simpleserial/simpleserial.h"

/// This function will handle the 'p' command send from the capture board.
/// It returns the squared version of the scmd given.
/// It does this in approximately equal time, which allows us to see clear
/// differences between different scmd values.
uint8_t test(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t *buf)
{
  volatile uint8_t result = 0;

  // Start measurement.
  trigger_high();

  // Cause more clock cycles to happen the higher the scmd is
  // We need 'volatile' here because we don't want the compiler to optimize the
  // loop out.
  //for (volatile uint8_t i = 0; i < 255; i++) {
  //  if (i == scmd) {
  //      result = scmd * scmd;
  //  }
  //}

  result = rust_test(32, 32);

  // Stop measurement.
  trigger_low();

  // For now we can just return the result back to the user.
  uint8_t buff[1] = { result };
  simpleserial_put('r', 1, buff);

  return 0;
}

uint8_t gen_key(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t *buf)
{
  volatile uint8_t result = 0;

  // Start measurement.
  trigger_high();

  result = rust_genkey();

  // Stop measurement.
  trigger_low();

  // For now we can just return the result back to the user.
  uint8_t buff[1] = { result };
  simpleserial_put('r', 1, buff);

  return 0;
}

int main(void) {
  // Setup the specific chipset.
  platform_init();
  // Setup serial communication line.
  init_uart();
  // Setup measurement trigger.
  trigger_setup();

  simpleserial_init();

  // Simpleserial_addcmd creates a listener that reacts upon a given command:
  // simpleserial_addcmd(char, unsigned int, (uin8_t, uin8_t, uin8_t, uin8_t*) -> uin8_t)
  // char: command target needs to listen for 'v' and 'w' are reserved
  // unsigned int: amount of data bytes expected, max is 192 bytes
  // (uin8_t, uin8_t, uin8_t, uin8_t*) -> uin8_t): function that takes cmd, subcommand, databuffer length and
  // the data buffer
  simpleserial_addcmd('t', 16, test);
  simpleserial_addcmd('g', 16, gen_key);

  // What for the capture board to send commands and handle them.
  while (1)
    simpleserial_get();
}