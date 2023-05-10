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

    with open("captured_traces/" + filename, "r") as filehandle:
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
    masked_deep_welsh = calc_welsh_t("fpc_mul_traces_masked_deep.txt")

    plt.plot(unmasked_welsh)
    plt.plot(masked_welsh)
    plt.plot(masked_deep_welsh)
    plt.grid()
    plt.show()

def do_fpr_mul_leak_test():
    unmasked_welsh = calc_welsh_t("fpr_mul_traces_1000.txt")
    masked_welsh = calc_welsh_t("fpr_mul_traces_masked_1000.txt")
    #masked_deep_welsh = calc_welsh_t("fpr_mul_traces_masked_deep_1000.txt")


    figure, axis = plt.subplots(2, 1)
    axis[0].plot(unmasked_welsh)
    axis[0].set_title("Unmasked")
    axis[0].set_ylim([-42, 42])
    axis[0].axhline(y= 4.892, color = 'r', linestyle = ':')
    axis[0].axhline(y= -4.892, color = 'r', linestyle = ':')
    axis[0].grid()

    axis[1].plot(masked_welsh)
    axis[1].set_title("Masked")
    axis[1].set_ylim([-42, 42])
    axis[1].axhline(y= 4.892, color = 'r', linestyle = ':')
    axis[1].axhline(y= -4.892, color = 'r', linestyle = ':')
    axis[1].grid()
    #
    # axis[2].plot(masked_deep_welsh)
    # axis[2].set_title("Deep masking")
    # axis[2].set_ylim([-42, 42])
    # axis[2].grid()
    #
    # plt.show()

    #plt.plot(unmasked_welsh)
    #plt.plot(masked_welsh)
    #plt.plot(masked_deep_welsh)
    #ax = plt.gca()
    #ax.set_ylim([-42, 42])
    #plt.grid()
    plt.show()

def do_fpr_add_leak_test():
    alpha = 5.327

    unmasked_welsh = calc_welsh_t("fpr_add_traces_10000.txt")
    masked_welsh = calc_welsh_t("fpr_add_traces_masked_10000.txt")
    deep_welsh = calc_welsh_t("fpr_add_traces_masked_1000_increased_range.txt")
    # deep_welsh = calc_welsh_t("fpr_add_traces_masked_deep_10000.txt")
    #deep_welsh_10000 = calc_welsh_t("fpr_add_traces_masked_deep_10000.txt")

    figure, axis = plt.subplots(3, 1)
    axis[0].plot(unmasked_welsh)
    axis[0].set_title("Unmasked")
    axis[0].set_ylim([-42, 42])
    axis[0].axhline(y= alpha, color = 'r', linestyle = ':')
    axis[0].axhline(y= -alpha, color = 'r', linestyle = ':')
    axis[0].grid()

    axis[1].plot(masked_welsh)
    axis[1].set_title("Masked")
    axis[1].set_ylim([-42, 42])
    axis[1].axhline(y= alpha, color = 'r', linestyle = ':')
    axis[1].axhline(y= -alpha, color = 'r', linestyle = ':')
    axis[1].grid()

    axis[2].plot(deep_welsh)
    axis[2].set_title("Deep")
    axis[2].set_ylim([-42, 42])
    axis[2].axhline(y= alpha, color = 'r', linestyle = ':')
    axis[2].axhline(y= -alpha, color = 'r', linestyle = ':')
    axis[2].grid()

    # axis[3].plot(deep_welsh_10000)
    # axis[3].set_title("Deep 10000")
    # axis[3].set_ylim([-42, 42])
    # axis[3].axhline(y= 4.892, color = 'r', linestyle = ':')
    # axis[3].axhline(y= -4.892, color = 'r', linestyle = ':')
    # axis[3].grid()
    plt.show()

def quick_test_please_no_snap():
    traces = {
        "masked": [],
        "rand": []
    }

    welsh_t_arr = []

    with open("captured_traces/sign_tree_masked_1000_masked.txt", "r") as filehandle:
        decoded = json.load(filehandle)

        traces["fix"] = np.array(decoded["masked"])

    with open("captured_traces/sign_tree_masked_1000_random.txt", "r") as filehandle:
        decoded = json.load(filehandle)

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

    plt.plot(welsh_t_arr)
    ax = plt.gca()
    ax.set_ylim([-200, 200])
    plt.grid()
    plt.show()

#do_fpc_mul_leak_test()
#do_fpr_mul_leak_test()
do_fpr_add_leak_test()
#do_fpr_mul_leak_test()
#quick_test_please_no_snap()