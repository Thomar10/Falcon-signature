import chipwhisperer as cw
import os
from chipwhisperer.capture.api.programmers import STM32FProgrammer
import matplotlib.pyplot as plt
import time
import struct

try:
    if not scope.connectStatus:
        scope.con()
except NameError:
    scope = cw.scope()

scope.default_setup()

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

def write_fpr(fpr):
    val_bytes = fpr.to_bytes(8, byteorder="little", signed=False)
    data_arr = [len(val_bytes)] + list(val_bytes)
    data = bytearray(data_arr)

    target.write(data)

def write_float(f):
    val_bytes = bytearray(struct.pack("d", f))
    data_arr = [len(val_bytes)] + list(val_bytes)
    data = bytearray(data_arr)

    target.write(data)

def get_avg_fft_trace(iterations, first_fpr):
    traces = []

    for i in range(iterations):
        scope.arm()
        target.flush()

        #write_fpr(first_fpr)
        write_float(first_fpr)

        time.sleep(1)

        ret = scope.capture()
        trace = scope.get_last_trace()

        traces.append(trace)

        returned_data = target.read()
        returned_val = int.from_bytes(bytes(returned_data, 'latin1'), "little")

        print("Iteration " + str(i) + " done. Result: " + str(returned_val))

    return calc_avg_trace(traces)

def do_fft_trace():
    iterations = 400

    #FPR_ONE = 4607182418800017408
    #FPR_TWO = 4611686018427387904
    FPR_ONE = float(1)
    FPR_TWO = float(2)


    one_trace = get_avg_fft_trace(iterations, FPR_ONE)
    two_trace = get_avg_fft_trace(iterations, FPR_TWO)

    #print(avg_trace)
    plt.plot(one_trace)
    plt.plot(two_trace)
    plt.show()

def calc_avg_trace(traces):
    avg_trace = []

    for i in range(len(traces[0])):
        avg = 0

        for j in range(len(traces)):
            avg += traces[j][i]

        avg = avg / len(traces)

        avg_trace.append(avg)

    return avg_trace

def do_write_test():
    test_val = 123456
    val_bytes = test_val.to_bytes(8, byteorder="little", signed=False)
    data_arr = [len(val_bytes)] + list(val_bytes)
    data = bytearray(data_arr)

    target.write(data)

    time.sleep(1)

    returned_data = target.read()
    returned_bytes = bytearray(returned_data, "latin1")
    returned_val = int.from_bytes(returned_bytes[1:], byteorder="little", signed=False)

    print(returned_val)

#do_write_test()
do_fft_trace()