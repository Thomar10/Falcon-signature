from base64 import decode
from cmath import sqrt
from inspect import trace
import json
import numpy as np
import matplotlib.pyplot as plt

def do_fpc_mul_leak_test():
    traces = {
        "fix": [],
        "rand": []
    }

    welsh_t_arr = []

    with open("fpc_mul_traces.txt", "r") as filehandle:
        decoded = json.load(filehandle)

        traces["fix"] = np.array(decoded["fix"])
        traces["rand"] = np.array(decoded["rand"])

    #plt.plot(traces["fix"][0])
    #plt.show()

    #print(len(traces["fix"]))

    #print(len(traces["fix"][0]))
    #print(len(traces["fix"][:, 1]))

    for i in range(len(traces["fix"][0])):
        Na = len(traces["fix"])
        Nb = len(traces["rand"])
        Xa = np.mean(traces["fix"][:, i])
        Xb = np.mean(traces["rand"][:, i])
        Sa = np.std(traces["fix"][:, i])
        Sb = np.std(traces["rand"][:, i])

        t = (Xa - Xb) / sqrt((Sa * Sa) / Na + (Sb * Sb) / Nb)

        welsh_t_arr.append(t)

        #print("Xa:", str(Xa))

    plt.plot(welsh_t_arr)
    plt.show()

do_fpc_mul_leak_test()