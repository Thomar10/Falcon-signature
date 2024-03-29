import chipwhisperer as cw
import os
from chipwhisperer.capture.api.programmers import STM32FProgrammer
import time
import struct
from random import SystemRandom
import json
import numpy as np
from json import JSONEncoder

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
program_hex_path = os.path.join(dir, r"main.hex") #Update accordingly

cw.program_target(scope, program, program_hex_path)

cryptogen = SystemRandom()

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
    for _ in range(10):
        test_val = 123456
        val_bytes = test_val.to_bytes(8, byteorder="little", signed=False)
        data_arr = [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(1)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        returned_val = int.from_bytes(returned_bytes, byteorder="little", signed=False)

        print(returned_val)

class NumpyArrayEncoder(JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        return JSONEncoder.default(self, obj)

def capture_trace(data):
    scope.arm()
    target.flush()

    target.write(data)

    while True:
        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")

        if len(returned_bytes) != 0:
            print("Result:", str(returned_bytes))

            ret = scope.capture()

            if ret:
                print("Error occured. Retrying")
                return capture_trace(data)

            trace = scope.get_last_trace()
            return trace

        time.sleep(0.1)

def do_sign_test(cmd=16, filename="sign_tree", iterations=1000):

    scope.clock.adc_src = "clkgen_x4"
    scope.adc.decimate = 2
    scope.adc.samples = 95000

    traces = {
        "fix": [],
        "rand": []
    }

    time.sleep(1)

    for i in range(iterations):
        print("Iteration:", str(i))

        # Fixed key

        type = 0
        seed = os.urandom(8)
        salt = os.urandom(40)
        data_arr = [cmd] + [type] + [len(seed) + len(salt) + 1] + list(seed) + list(salt)
        data = bytearray(data_arr)

        fix_trace = capture_trace(data)
        traces["fix"].append(fix_trace)

        # Random key
        type = 1
        seed = os.urandom(8)
        salt = os.urandom(40)
        data_arr = [cmd] + [type] + [len(seed) + len(salt) + 1] + list(seed) + list(salt)
        data = bytearray(data_arr)

        rand_trace = capture_trace(data)
        traces["rand"].append(rand_trace)

    print("**********TEST DONE**********")

    #Write traces to file
    with open("new_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
       json.dump(traces, filehandle, cls=NumpyArrayEncoder)

def do_fft_test(type=11, filename="fft", iterations=1000, samples=30000):
    scope.clock.adc_src = "clkgen_x4"
    scope.adc.decimate = 2
    scope.adc.samples = samples

    traces = {
        "fix": [],
        "rand": []
    }

    offset = 0

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        scope.arm()
        target.flush()

        data_arr = [type] + [2] + [0] + [offset]
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["fix"].append(trace)

        #Random test
        scope.arm()
        target.flush()

        data_arr = [type] + [2] + [1] + [offset]
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["rand"].append(trace)

    #Write traces to file
    with open("new_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

    print("**********TEST DONE**********")

def do_sub_test(type=7, filename="secure_ursh", iterations=1000):
    scope.clock.adc_src = "clkgen_x4"
    scope.adc.decimate = 2
    scope.adc.samples = 10000

    traces = {
        "fix": [],
        "rand": []
    }

    fix_a_val = float(68.20750458284908)
    fix_shift_val = 4

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        fix_a_rand = float(cryptogen.random() * 256 - 128)
        fix_shift_rand = int(cryptogen.random() * 16)

        val_bytes = bytearray(struct.pack("2d", fix_a_val, fix_a_rand))
        data_arr = [type] + [len(val_bytes) + 2] + list(val_bytes) + [fix_shift_val] + [fix_shift_rand]
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["fix"].append(trace)

        #Random test
        rand_a_val = float(cryptogen.random() * 256 - 128)
        rand_a_rand = float(cryptogen.random() * 256 - 128)

        rand_shift_val = int(cryptogen.random() * 16)
        rand_shift_rand = int(cryptogen.random() * 16)

        val_bytes = bytearray(struct.pack("2d", rand_a_val, rand_a_rand))
        data_arr = [type] + [len(val_bytes) + 2] + list(val_bytes) + [rand_shift_val] + [rand_shift_rand]
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["rand"].append(trace)

    #Write traces to file
    with open("new_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

    print("**********TEST DONE**********")

def do_simple_test(type, filename, iterations=1000):
    scope.clock.adc_src = "clkgen_x4"
    scope.adc.decimate = 2
    scope.adc.samples = 10000

    traces = {
        "fix": [],
        "rand": []
    }

    fix_a_val = float(68.20750458284908)
    fix_b_val = float(-92.93250079435525)

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        fix_a_rand = float(cryptogen.random() * 256 - 128)
        fix_b_rand = float(cryptogen.random() * 256 - 128)

        val_bytes = bytearray(struct.pack("4d", fix_a_val, fix_b_val, fix_a_rand, fix_b_rand))
        data_arr = [type] + [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["fix"].append(trace)

        #Random test
        rand_a_val = float(cryptogen.random() * 256 - 128)
        rand_b_val = float(cryptogen.random() * 256 - 128)
        rand_a_rand = float(cryptogen.random() * 256 - 128)
        rand_b_rand = float(cryptogen.random() * 256 - 128)

        val_bytes = bytearray(struct.pack("4d", rand_a_val, rand_b_val, rand_a_rand, rand_b_rand))
        data_arr = [type] + [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        trace = capture_trace(data)
        traces["rand"].append(trace)

    #Write traces to file
    with open("new_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

    print("**********TEST DONE**********")

def do_all_simple():
    print("STARTING ALL SIMPLE TESTS")
    do_simple_test(1, "fpr_add_traces", 1000)
    time.sleep(10)
    do_simple_test(2, "fpr_add_traces_masked", 1000)
    time.sleep(10)
    do_simple_test(3, "fpr_add_traces_masked_deep", 1000)
    time.sleep(10)
    do_simple_test(4, "fpr_mul_traces", 1000)
    time.sleep(10)
    do_simple_test(5, "fpr_mul_traces_masked", 1000)
    time.sleep(10)
    do_simple_test(6, "fpr_mul_traces_masked_deep", 1000)
    time.sleep(10)
    do_simple_test(1, "fpr_add_traces", 10000)
    time.sleep(10)
    do_simple_test(2, "fpr_add_traces_masked", 10000)
    time.sleep(10)
    do_simple_test(3, "fpr_add_traces_masked_deep", 10000)
    time.sleep(10)
    do_simple_test(4, "fpr_mul_traces", 10000)
    time.sleep(10)
    do_simple_test(5, "fpr_mul_traces_masked", 10000)
    time.sleep(10)
    do_simple_test(6, "fpr_mul_traces_masked_deep", 10000)
    time.sleep(10)

def test_clock():
    scope.clock.adc_src = "clkgen_x4"
    time.sleep(1)
    freq = scope.clock.adc_freq
    print("Current frequency", str(freq))
    srate = scope.clock.adc_rate
    print("Current sampling rate:", str(srate))
    adc_src = scope.clock.adc_src
    print("Current adc src", adc_src)

# do_all_simple()
# do_write_test()
# do_fft_trace()
# do_fpc_mul_masked_test()
# do_fpr_mul_test()
# do_fpr_mul_masked_test()
# do_fpr_add_test()
# do_sub_test(type=7, filename="ursh", iterations=1000)
# do_sub_test(type=8, filename="secure_ursh", iterations=1000)
# do_sub_test(type=9, filename="norm", iterations=1000)
# do_sub_test(type=10, filename="secure_norm", iterations=1000)
#
# do_fft_test(type=11, filename="fft", iterations=1000, samples=20000)
# do_fft_test(type=12, filename="fft_masked", iterations=1000, samples=30000)
# do_fft_test(type=13, filename="fft_masked_deep", iterations=1000, samples=95000)

# test_clock()

# do_simple_test(2, "fpr_add_traces_slow_masked", 1000)

# do_fft_test(type=14, filename="poly_mul_fft", iterations=1000, samples=20000)
# do_fft_test(type=15, filename="poly_mul_fft_masked", iterations=1000, samples=30000)
# do_sign_test(cmd=16, filename="ffSamplingLOGN8", iterations=1000)
# do_sign_test(cmd=17, filename="ffSamplingLOGN8_masked", iterations=1000)