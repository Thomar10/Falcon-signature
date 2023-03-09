import numpy as np
import chipwhisperer as cw
import os
from chipwhisperer.capture.api.programmers import STM32FProgrammer
import matplotlib.pyplot as plt

try:
    if not scope.connectStatus:
        scope.con()
except NameError:
    scope = cw.scope()

scope.default_setup()
target_type = cw.targets.SimpleSerial2
target = cw.target(scope, target_type)

print("INFO: Found ChipWhisperer")

program = STM32FProgrammer

dir = os.path.dirname(os.path.realpath(__file__))
program_hex_path = os.path.join(dir, r"simpleserial-target-CW308_STM32F3.hex") #Update accordingly

cw.program_target(scope, program, program_hex_path)

scope.arm()
target.flush()

# Dummy data
data = bytearray([0x42] * 16)

# Send command to trigger code execution
target.send_cmd('p', 0x80, data)

# Fetch trace
ret = scope.capture()
trace = scope.get_last_trace()

# Print the returned data
returned_data = target.read_cmd('r')
print(returned_data)
ack = target.read_cmd('e')

# Plot the trace
plt.plot(trace)
plt.show()