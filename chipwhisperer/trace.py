import chipwhisperer as cw
import os
from chipwhisperer.capture.api.programmers import STM32FProgrammer
import matplotlib.pyplot as plt
import time
import struct
import random
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
#scope.clock.clkgen_freq = 200000000
#print(scope.clock.adc_rate)

program = STM32FProgrammer

dir = os.path.dirname(os.path.realpath(__file__))
program_hex_path = os.path.join(dir, r"unmasked.hex") #Update accordingly

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

def do_fpc_mul_test():
    traces = {
        "fix": [],
        "rand": []
    }

    iterations = 10

    fix_a_re = float(68.20750458284908)
    fix_a_im = float(-57.48737600599283)
    fix_b_re = float(-92.93250079435525)
    fix_b_im = float(42.45470502022772)

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        scope.arm()
        target.flush()

        val_bytes = bytearray(struct.pack("4d", fix_a_re, fix_a_im, fix_b_re, fix_b_im))
        data_arr = [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (a, b) = struct.unpack("2d", returned_bytes)

        print("Fixed result: a: " + str(a) + " b: " + str(b))

        #Random test
        rand_a_re = float(random.uniform(-100, 100))
        rand_a_im = float(random.uniform(-100, 100))
        rand_b_re = float(random.uniform(-100, 100))
        rand_b_im = float(random.uniform(-100, 100))

        scope.arm()
        target.flush()

        val_bytes = bytearray(struct.pack("4d", rand_a_re, rand_a_im, rand_b_re, rand_b_im))
        data_arr = [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (a, b) = struct.unpack("2d", returned_bytes)

        print("Random result: a: " + str(a) + " b: " + str(b))

    #Write traces to file
    with open("captured_traces/fpc_mul_traces.txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

def do_fpc_mul_masked_test():
    traces = {
        "fix": [],
        "rand": []
    }

    iterations = 10

    fix_a_re_val = float(68.20750458284908)
    fix_a_im_val = float(-57.48737600599283)
    fix_b_re_val = float(-92.93250079435525)
    fix_b_im_val = float(42.45470502022772)

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        scope.arm()
        target.flush()

        fix_a_re_rand = float(random.uniform(-100, 100))
        fix_a_im_rand = float(random.uniform(-100, 100))
        fix_b_re_rand = float(random.uniform(-100, 100))
        fix_b_im_rand = float(random.uniform(-100, 100))

        val_bytes = bytearray(struct.pack("8d", fix_a_re_val, fix_a_im_val, fix_b_re_val, fix_b_im_val, fix_a_re_rand, fix_a_im_rand, fix_b_re_rand, fix_b_im_rand))
        data_arr = [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (a, b) = struct.unpack("2d", returned_bytes)

        print("Fixed result: a: " + str(a) + " b: " + str(b))

        #Random test
        rand_a_re_val = float(random.uniform(-100, 100))
        rand_a_im_val = float(random.uniform(-100, 100))
        rand_b_re_val = float(random.uniform(-100, 100))
        rand_b_im_val = float(random.uniform(-100, 100))
        rand_a_re_rand = float(random.uniform(-100, 100))
        rand_a_im_rand = float(random.uniform(-100, 100))
        rand_b_re_rand = float(random.uniform(-100, 100))
        rand_b_im_rand = float(random.uniform(-100, 100))

        scope.arm()
        target.flush()

        val_bytes = bytearray(struct.pack("8d", rand_a_re_val, rand_a_im_val, rand_b_re_val, rand_b_im_val, rand_a_re_rand, rand_a_im_rand, rand_b_re_rand, rand_b_im_rand))
        data_arr = [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (a, b) = struct.unpack("2d", returned_bytes)

        print("Random result: a: " + str(a) + " b: " + str(b))

    #Write traces to file
    with open("captured_traces/fpc_mul_traces_masked.txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)


def do_sign_test():

    scope.adc.samples = 20000

    traces = {
        "fix": [],
        "rand": []
    }

    iterations = 1000

    for i in range(iterations):
        print("Iteration:", str(i))

        # Fixed key

        scope.arm()
        target.flush()

        type = 0
        seed = os.urandom(8)
        salt = os.urandom(40)
        data_arr = [type] + [len(seed) + len(salt)] + list(seed) + list(salt)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(5)

        ret = scope.capture()
        trace = scope.get_last_trace()

        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")

        print("Result:", str(returned_bytes))

        # Random key
        scope.arm()
        target.flush()

        type = 1
        seed = os.urandom(8)
        salt = os.urandom(40)
        data_arr = [type] + [len(seed) + len(salt)] + list(seed) + list(salt)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(30)
        #Falcon 8 needs 25 seconds to create a random key :O

        ret = scope.capture()
        trace = scope.get_last_trace()

        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")

        print("Result:", str(returned_bytes))

    #Write traces to file
    with open("captured_traces/sign_tree_masked_1000.txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

def do_fft_test(type=11, filename="fft", iterations=1000):
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

        target.write(data)

        time.sleep(10)

        ret = scope.capture()
        trace = scope.get_last_trace()

        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Fixed result: " + str(c))

        #Random test
        scope.arm()
        target.flush()

        data_arr = [type] + [2] + [1] + [offset]
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(10)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Random result: " + str(c))

    #Write traces to file
    with open("captured_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

    print("**********TEST DONE**********")

def do_sub_test(type=7, filename="secure_ursh", iterations=1000):
    traces = {
        "fix": [],
        "rand": []
    }

    fix_a_val = float(68.20750458284908)
    fix_shift_val = 4

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        scope.arm()
        target.flush()

        fix_a_rand = float(cryptogen.random() * 256 - 128)
        fix_shift_rand = int(cryptogen.random() * 16)

        val_bytes = bytearray(struct.pack("2d", fix_a_val, fix_a_rand))
        data_arr = [type] + [len(val_bytes) + 2] + list(val_bytes) + [fix_shift_val] + [fix_shift_rand]
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(0.1)

        ret = scope.capture()
        trace = scope.get_last_trace()

        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Fixed result: " + str(c))

        #Random test
        scope.arm()
        target.flush()

        rand_a_val = float(cryptogen.random() * 256 - 128)
        rand_a_rand = float(cryptogen.random() * 256 - 128)

        rand_shift_val = int(cryptogen.random() * 16)
        rand_shift_rand = int(cryptogen.random() * 16)

        val_bytes = bytearray(struct.pack("2d", rand_a_val, rand_a_rand))
        data_arr = [type] + [len(val_bytes) + 2] + list(val_bytes) + [rand_shift_val] + [rand_shift_rand]
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(0.1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Random result: " + str(c))

    #Write traces to file
    with open("captured_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
        json.dump(traces, filehandle, cls=NumpyArrayEncoder)

    print("**********TEST DONE**********")
def do_simple_test(type, filename, iterations=1000):
    traces = {
        "fix": [],
        "rand": []
    }

    fix_a_val = float(68.20750458284908)
    fix_b_val = float(-92.93250079435525)

    for i in range(iterations):
        print("Iteration:", str(i))

        #Fixed test
        scope.arm()
        target.flush()

        fix_a_rand = float(cryptogen.random() * 256 - 128)
        fix_b_rand = float(cryptogen.random() * 256 - 128)

        val_bytes = bytearray(struct.pack("4d", fix_a_val, fix_b_val, fix_a_rand, fix_b_rand))
        data_arr = [type] + [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(0.1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["fix"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Fixed result: " + str(c))

        #Random test
        rand_a_val = float(cryptogen.random() * 256 - 128)
        rand_b_val = float(cryptogen.random() * 256 - 128)
        rand_a_rand = float(cryptogen.random() * 256 - 128)
        rand_b_rand = float(cryptogen.random() * 256 - 128)

        scope.arm()
        target.flush()

        val_bytes = bytearray(struct.pack("4d", rand_a_val, rand_b_val, rand_a_rand, rand_b_rand))
        data_arr = [type] + [len(val_bytes)] + list(val_bytes)
        data = bytearray(data_arr)

        target.write(data)

        time.sleep(0.1)

        ret = scope.capture()
        trace = scope.get_last_trace()
        traces["rand"].append(trace)

        returned_data = target.read()
        returned_bytes = bytearray(returned_data, "latin1")
        (c) = struct.unpack("d", returned_bytes)

        print("Random result: " + str(c))

    #Write traces to file
    with open("captured_traces/" + filename + "_" + str(iterations) + ".txt", "w") as filehandle:
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

#do_all_simple()
#do_write_test()
#do_fft_trace()
#do_fpc_mul_masked_test()
#do_fpr_mul_test()
#do_fpr_mul_masked_test()
#do_fpr_add_test()
#do_sub_test(type=7, filename="ursh", iterations=1000)
#do_sub_test(type=8, filename="secure_ursh", iterations=1000)
#do_sub_test(type=9, filename="norm", iterations=1000)
#do_sub_test(type=10, filename="secure_norm", iterations=1000)

#do_fft_test(type=11, filename="fft", iterations=1000)
#do_fft_test(type=12, filename="fft_masked", iterations=1000)
#do_fft_test(type=13, filename="fft_masked_deep", iterations=1000)

do_sign_test()
#do_sign_masked_test_masked()
#do_sign_masked_test_random()