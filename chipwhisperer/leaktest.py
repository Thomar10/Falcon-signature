from base64 import decode
from cmath import sqrt
from inspect import trace
import json
import numpy as np
import matplotlib.pyplot as plt
import matplotlib as mpl

mpl.rcParams["savefig.directory"] = "C:\\Users\\Malth\\OneDrive - Aarhus Universitet\\10. Semester\\Power Trace Graphs"

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

def do_graph(title, filename, alpha=4.892, limit=5000, ylim=42):
    welsh = calc_welsh_t(filename)

    fig, ax = plt.subplots(figsize=(20, 7))

    ax.plot(welsh[:limit])
    ax.set_title(title, fontsize=24)
    ax.set_ylabel("T-value", fontsize=18)
    ax.set_xlabel("# Power Trace Sample", fontsize=18)
    ax.set_ylim([-ylim, ylim])
    ax.axhline(y= alpha, color = 'r', linestyle = ':')
    ax.axhline(y= -alpha, color = 'r', linestyle = ':')
    ax.grid()
    props = dict(boxstyle="round", facecolor="wheat", alpha=0.5)
    ax.text(0.88, 0.95, "Threshold: " + str(alpha), transform=ax.transAxes, fontsize=18, verticalalignment="top", bbox=props)
    fig.tight_layout()
    #plt.show()

    filename = title
    filename = filename.replace(",", "")
    filename = filename.replace(".", "")
    filename = filename.replace(" ", "_")
    filename = filename.lower()

    plt.savefig("C:\\Users\\Malth\\OneDrive - Aarhus Universitet\\10. Semester\\Power Trace Graphs\\" + filename + ".png", dpi=400, format="png")
    print("Plot saved as: " + filename)
    plt.show()


# do_graph("Unmasked FPR Addition, 1,000 Traces", "fpr_add_traces_1000.txt", limit=600)
# do_graph("Arithmetic Masking of FPR Addition, 1,000 Traces", "fpr_add_traces_masked_1000.txt", limit=1800)
# do_graph("Boolean Masking of FPR Addition, 1,000 Traces", "fpr_add_traces_masked_deep_1000.txt", limit=2500)
# do_graph("Unmasked FPR Addition, 10,000 Traces", "fpr_add_traces_10000.txt", alpha=5.327, limit=600, ylim=115)
# do_graph("Arithmetic Masking of FPR Addition, 10,000 Traces", "fpr_add_traces_masked_10000.txt", alpha=5.327, limit=1800)
# do_graph("Boolean Masking of FPR Addition, 10,000 Traces", "fpr_add_traces_masked_deep_10000.txt", alpha=5.327, limit=2500, ylim=135)
#
# do_graph("Unmasked FPR Multiplication, 1,000 Traces", "fpr_mul_traces_1000.txt", limit=400)
# do_graph("Arithmetic Masking of FPR Multiplication, 1,000 Traces", "fpr_mul_traces_masked_1000.txt", limit=2500)
# do_graph("Boolean Masking of FPR Multiplication, 1,000 Traces", "fpr_mul_traces_masked_deep_1000.txt", limit=4000)
# do_graph("Unmasked FPR Multiplication, 10,000 Traces", "fpr_mul_traces_10000.txt", alpha=5.327, limit=400, ylim=115)
# do_graph("Arithmetic Masking of FPR Multiplication, 10,000 Traces", "fpr_mul_traces_masked_10000.txt", alpha=5.327, limit=2500)
# do_graph("Boolean Masking of FPR Multiplication, 10,000 Traces", "fpr_mul_traces_masked_deep_10000.txt", alpha=5.327, limit=4000, ylim=65)

# do_graph("Secure Ursh Boolean Mask, 1,000 Traces", "secure_ursh_1000.txt")
# do_graph("Unmasked Ursh, 1,000 Traces", "ursh_1000.txt")
# do_graph("Secure Ursh Boolean Mask, 10,000 Traces", "secure_ursh_10000.txt", alpha=5.327)

# do_graph("Unmasked Norm, 1,000 Traces", "norm_1000.txt")
# do_graph("Secure Norm Boolean Mask, 1,000 Traces", "secure_norm_1000.txt")

# do_graph("Unmasked FFT, 1,000 Traces", "fft_1000.txt")
# do_graph("Masked FFT, 1,000 Traces", "fft_masked_1000.txt")

#do_fpc_mul_leak_test()
#do_fpr_mul_leak_test()
#do_fpr_add_leak_test()
#do_fpr_mul_leak_test()
#quick_test_please_no_snap()