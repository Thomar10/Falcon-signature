from cmath import sqrt
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

    with open("new_traces/" + filename, "r") as filehandle:
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

    plt.savefig("C:\\Users\\Malth\\OneDrive - Aarhus Universitet\\10. Semester\\New Graphs\\" + filename + ".png", dpi=400, format="png")
    print("Plot saved as: " + filename)
    plt.show()


# do_graph("Unmasked FPR Addition, 1,000 Traces", "fpr_add_traces_1000.txt", limit=600, ylim=70)
# do_graph("Arithmetic Masking of FPR Addition, 1,000 Traces", "fpr_add_traces_masked_1000.txt", limit=900)
# do_graph("Boolean Masking of FPR Addition, 1,000 Traces", "fpr_add_traces_masked_deep_1000.txt", limit=3000, ylim=80)
# do_graph("Unmasked FPR Addition, 10,000 Traces", "fpr_add_traces_10000.txt", alpha=5.327, limit=600, ylim=210)
# do_graph("Arithmetic Masking of FPR Addition, 10,000 Traces", "fpr_add_traces_masked_10000.txt", alpha=5.327, limit=900)
# do_graph("Boolean Masking of FPR Addition, 10,000 Traces", "fpr_add_traces_masked_deep_10000.txt", alpha=5.327, limit=3000, ylim=260)
#
# do_graph("Unmasked FPR Multiplication, 1,000 Traces", "fpr_mul_traces_1000.txt", limit=600, ylim=80)
# do_graph("Arithmetic Masking of FPR Multiplication, 1,000 Traces", "fpr_mul_traces_masked_1000.txt", limit=1800)
# do_graph("Boolean Masking of FPR Multiplication, 1,000 Traces", "fpr_mul_traces_masked_deep_1000.txt", limit=6000)
# do_graph("Unmasked FPR Multiplication, 10,000 Traces", "fpr_mul_traces_10000.txt", alpha=5.327, limit=600, ylim=250)
# do_graph("Arithmetic Masking of FPR Multiplication, 10,000 Traces", "fpr_mul_traces_masked_10000.txt", alpha=5.327, limit=1800)
# do_graph("Boolean Masking of FPR Multiplication, 10,000 Traces", "fpr_mul_traces_masked_deep_10000.txt", alpha=5.327, limit=6000, ylim=90)

# do_graph("Secure Ursh Boolean Mask, 1,000 Traces", "secure_ursh_1000.txt", limit=1200)
# do_graph("Unmasked Ursh, 1,000 Traces", "ursh_1000.txt", limit=200)
# do_graph("Secure Ursh Boolean Mask, 10,000 Traces", "secure_ursh_10000.txt", alpha=5.327)

# do_graph("Unmasked Norm, 1,000 Traces", "norm_1000.txt", limit=300)
# do_graph("Secure Norm Boolean Mask, 1,000 Traces", "secure_norm_1000.txt", ylim=120, limit=4000)

# do_graph("Unmasked FFT, 1,000 Traces", "fft_1000.txt", limit=20000, ylim=250)
# do_graph("Arithmetic Masking of FFT, 1,000 Traces", "fft_masked_1000.txt", limit=50000)
# do_graph("Boolean Masking of FFT, 1,000 Traces", "fft_masked_deep_1000.txt", ylim=140, limit=75000)

# do_graph("Unmasked FFT Multiplication, 1,000 Traces", "poly_mul_fft_1000.txt", limit=40000, ylim=175)
# do_graph("Arithmetic Masking of FFT Multiplication, 1,000 Traces", "poly_mul_fft_masked_1000.txt", limit=40000)

# do_graph("Unmasked Sign Tree, 1,000 Traces", "sign_tree_1000.txt", limit=20000)
# do_graph("Masked Sign Tree, 1,000 Traces", "sign_tree_masked_1000.txt", limit=20000)

# do_graph("Unmasked ffSampling LOGN=8, 1,000 Traces", "ffSampling_1000.txt", limit=15000)
# do_graph("Masked ffSampling LOGN=8, 1,000 Traces", "ffSampling_masked_1000.txt", limit=15000)

# do_graph("Unmasked ffSampling LOGN=2, 1,000 Traces", "ffSamplingLOGN2_1000.txt", limit=90000, ylim=70)
# do_graph("Masked ffSampling LOGN=2, 1,000 Traces", "ffSamplingLOGN2_masked_1000.txt", limit=90000, ylim=60)

# do_graph("Unmasked Sign Tree New, 100 Traces", "sign_tree_100.txt", limit=4000000)
# do_graph("Masked Sign Tree New, 1,000 Traces", "sign_tree_masked_1000_new.txt", limit=20000)

# do_graph("Test Sign Stream, 5 Traces", "sign_tree_5.txt", limit=3000000)
# do_graph("Test FFT, 1000 Traces", "fft_masked_very_large_100.txt", limit=3000000)

# do_graph("Test of FPR_ADD_MASKED - Slow Clock, 1,000 Traces", "fpr_add_traces_slow_masked_1000.txt", limit=95_000)

#do_fpc_mul_leak_test()
#do_fpr_mul_leak_test()
#do_fpr_add_leak_test()
#do_fpr_mul_leak_test()
#quick_test_please_no_snap()