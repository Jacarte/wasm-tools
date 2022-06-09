import json
import sys
import matplotlib.pyplot as plt

USE_TIME = False

if __name__ == "__main__":
    typ = sys.argv[1]
    key = sys.argv[2]
    dkey = sys.argv[3]
    datas = sys.argv[4::2]
    names = sys.argv[5::2]

    for d, name in zip(datas, names):
        ls = json.loads(open(d, 'r').read())
        print(name, ls, ls[key], ls[key][typ])
        ys = [0]
        xs = [0]

        ys += [100*y1/y0 for y1, y0 in zip(ls[key][typ][dkey], ls[key][typ]['total'])] 
        xs  += ls[key]['input_count'] if not USE_TIME else  ls[key]['time']

        plt.plot(xs, ys, '.-', label=name)
    
    plt.title(key)
    plt.ylabel("Covered lines (%)")
    plt.xlabel("Time (minutes)" if USE_TIME else "Number of inputs")
    plt.legend()
    # plt.show()
    plt.savefig(f"time{USE_TIME}.png")
