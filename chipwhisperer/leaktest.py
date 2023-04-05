from base64 import decode
from cmath import sqrt
from inspect import trace
import json
import numpy as np
import matplotlib.pyplot as plt

def calc_welsh_t(filename):
    traces = {
        "fix": [],
        "rand": []
    }

    welsh_t_arr = []

    with open(filename, "r") as filehandle:
        decoded = json.load(filehandle)

        traces["fix"] = np.array(decoded["fix"])
        traces["rand"] = np.array(decoded["rand"])

    for i in range(len(traces["fix"][0])):
        Na = len(traces["fix"])
        Nb = len(traces["rand"])
        Xa = np.mean(traces["fix"][:, i])
        Xb = np.mean(traces["rand"][:, i])
        Sa = np.std(traces["fix"][:, i])
        Sb = np.std(traces["rand"][:, i])

        t = (Xa - Xb) / sqrt((Sa * Sa) / Na + (Sb * Sb) / Nb)

        welsh_t_arr.append(t)

    return welsh_t_arr

def do_fpc_mul_leak_test():

    unmasked_welsh = calc_welsh_t("fpc_mul_traces.txt")
    masked_welsh = calc_welsh_t("fpc_mul_traces_masked.txt")

    plt.plot(unmasked_welsh)
    plt.plot(masked_welsh)
    plt.show()

do_fpc_mul_leak_test()