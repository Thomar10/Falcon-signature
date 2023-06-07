This folder contains test to be run on the ChipWhisperer Pro using an STM32F415 target in order to capture power traces.

To run a test, the binaries must first be compiled for the target. This is done in 3 steps:

1. Clean old build files using ```cargo clean```
2. Build new .elf files in release mode using ```cargo build --release```
3. Compile .elf files to binary hex: ```arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabihf/release/main main.hex```

The hex file can now be uploaded to the ChipWhisperer using the python API. Examples of which can be found in trace.py

After a trace has been captured, a Welch t-test can be perfomed. Examples of which can be found in leaktest.py