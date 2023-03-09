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
uint8_t handle(uint8_t cmd, uint8_t scmd, uint8_t len, uint8_t *buf)
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

  result = test(32, 32);

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

  // Insert your handlers here.
  simpleserial_addcmd('p', 16, handle);

  // What for the capture board to send commands and handle them.
  while (1)
    simpleserial_get();
}