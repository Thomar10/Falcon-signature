import numpy as np
import chipwhisperer as cw
import os
from chipwhisperer.capture.api.programmers import STM32FProgrammer
import matplotlib.pyplot as plt
import time

try:
    if not scope.connectStatus:
        scope.con()
except NameError:
    scope = cw.scope()

scope.default_setup()
#target_type = cw.targets.SimpleSerial2

try:
    target = cw.target(scope)
except:
    print("INFO: Caught exception on reconnecting to target - attempting to reconnect to scope first.")
    print("INFO: This is a work-around when USB has died without Python knowing. Ignore errors above this line.")
    scope = cw.scope()
    target = cw.target(scope)

print("INFO: Found ChipWhisperer")

program = STM32FProgrammer

dir = os.path.dirname(os.path.realpath(__file__))
program_hex_path = os.path.join(dir, r"unmasked.hex") #Update accordingly

cw.program_target(scope, program, program_hex_path)

scope.arm()
target.flush()

# Dummy data
data = bytearray([0, 8, 1, 2, 3, 4, 5, 6, 7, 8])

# Send command to trigger code execution
#Trigger test
#target.send_cmd('t', 0x80, data)

target.write(data)

""" print("Waiting 10 seconds for board to do computation:")
for i in range(10):
    time.sleep(1)
    print("", str(9 - i)) """

# Fetch trace
ret = scope.capture()
trace = scope.get_last_trace()

# Print the returned data
#cmd = target.read(1)
returned_data = target.read()
returned_val = int.from_bytes(bytes(returned_data, 'latin1'), "little")
print("Bytes:", list(bytes(returned_data, 'latin1')))
print("Data:", returned_val)

# Plot the trace
plt.plot(trace)
plt.show()